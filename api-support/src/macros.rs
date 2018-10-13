#[macro_export]
macro_rules! async_request {
    ($client: expr, $request: expr, $type: ty) => {{
        //println!("{:?}", $request);
        let f = $client
            .execute($request.build().unwrap())
            .from_err::<$crate::error::Error>()
            .and_then(|mut res| {
                let status = res.status().as_u16();
                let success = res.status().is_success();
                let body = mem::replace(res.body_mut(), Decoder::empty());

                let mime = res
                    .headers()
                    .get("application/json")
                    .and_then(|ct| ct.to_str().ok())
                    .unwrap_or("application/json")
                    .to_string();

                let b = body
                    .concat2()
                    .from_err::<$crate::error::Error>()
                    .and_then(move |b| {
                        if !success {
                            let msg = String::from_utf8(b.to_vec()).unwrap();
                            return Err($crate::error::ErrorKind::Client(status, msg).into());
                        }

                        Ok($crate::utils::decode::<$type>(&mime, &b)?)
                    });

                Box::new(b)
            })
            .from_err();

        Box::new(f)
    }};
}
