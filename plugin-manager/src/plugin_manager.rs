use error::Result;

use error::ResultExt;
use libloading::{Library, Symbol};
use std::any::Any;
use std::ffi::OsStr;
use uuid::Uuid;

/// Declare a plugin type and its constructor.
///
/// # Notes
///
/// This works by automatically generating an `extern "C"` function with a
/// pre-defined signature and symbol name. Therefore you will only be able to
/// declare one plugin per library.
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut $crate::Plugin {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;

            let object = constructor();
            let boxed: Box<$crate::Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}

pub trait Plugin: Any + Send + Sync {
    /// Get a name describing the `Plugin`.
    fn name(&self) -> &'static str;
    fn on_plugin_unload(&self) {}
    fn on_plugin_load(&self) {}
}

struct Entry {
    id: Uuid,
    plugin: Box<Plugin>,
    path: String,
    //library: Library,
}

struct LibraryEntry {
    id: Uuid,
    library: Library,
}

pub struct PluginManager {
    plugins: Vec<Entry>,
    loaded_libraries: Vec<LibraryEntry>,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: Vec::new(),
            loaded_libraries: Vec::new(),
        }
    }

    pub unsafe fn load_plugin<P: AsRef<OsStr>>(&mut self, filename: P) -> Result<&Box<Plugin>> {
        type PluginCreate = unsafe fn() -> *mut Plugin;

        let lib = Library::new(filename.as_ref()).chain_err(|| "Unable to load the plugin")?;
        let id = Uuid::new_v4();

        // We need to keep the library around otherwise our plugin's vtable will
        // point to garbage. We do this little dance to make sure the library
        // doesn't end up getting moved.
        self.loaded_libraries.push(LibraryEntry {
            id: id,
            library: lib,
        });

        let lib = self.loaded_libraries.last().unwrap();

        let constructor: Symbol<PluginCreate> = lib
            .library
            .get(b"_plugin_create")
            .chain_err(|| "The `_plugin_create` symbol wasn't found.")?;
        let boxed_raw = constructor();

        let plugin = Box::from_raw(boxed_raw);
        debug!("Loaded plugin: {}", plugin.name());
        plugin.on_plugin_load();

        self.plugins.push(Entry {
            id: id,
            path: String::from(filename.as_ref().to_str().unwrap()),
            plugin: plugin,
        });

        let entry = self.plugins.last().unwrap();

        Ok(&entry.plugin)
    }

    pub fn plugin_with_name(&self, name: &str) -> Vec<&Box<dyn Plugin>> {
        let mut out = vec![];
        for plugin in &self.plugins {
            if plugin.plugin.name() == name {
                out.push(&plugin.plugin);
            }
        }
        out
    }

    pub fn plugin(&self, id: Uuid) -> Option<&Box<dyn Plugin>> {
        for plugin in &self.plugins {
            if plugin.id == id {
                return Some(&plugin.plugin);
            }
        }
        None
    }

    pub fn unload_plugin(&mut self, id: Uuid) {
        let found = (&self.plugins).into_iter().position(|k| k.id == id);
        if let Some(index) = found {
            self.plugins[index].plugin.on_plugin_unload();
            self.plugins.remove(index);
            let found = (&self.loaded_libraries)
                .into_iter()
                .position(|k| k.id == id);
            if let Some(index) = found {
                drop(&self.loaded_libraries[index]);
                self.loaded_libraries.remove(index);
            }
        }
    }

    pub fn unload(&mut self) {
        debug!("Unloading plugins");

        for entry in self.plugins.drain(..) {
            trace!("Firing on_plugin_unload for {:?}", entry.plugin.name());
            entry.plugin.on_plugin_unload();
        }

        for entry in self.loaded_libraries.drain(..) {
            drop(entry.library);
        }
    }
}
