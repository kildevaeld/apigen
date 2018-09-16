use plugins;

error_chain!{
    links {
        PluginManager(plugins::Error, plugins::ErrorKind);
    }
}
