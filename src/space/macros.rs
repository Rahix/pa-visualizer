#[macro_export]
macro_rules! shader_program {
    ($display:expr, $vert_file:expr, $frag_file:expr) => ({
        // Use this for debug
        let vert_src = {
            use ::std::io::Read;
            let mut buf = String::new();
            let mut f = ::std::fs::File::open(format!("src/space/{}", $vert_file)).unwrap();
            f.read_to_string(&mut buf).unwrap();

            buf
        };

        let frag_src = {
            use ::std::io::Read;
            let mut buf = String::new();
            let mut f = ::std::fs::File::open(format!("src/space/{}", $frag_file)).unwrap();
            f.read_to_string(&mut buf).unwrap();

            buf
        };

        glium::Program::from_source($display,
                &vert_src,
                &frag_src,
                None).unwrap()
        // Use this for release
        /*glium::Program::from_source($display,
            include_str!($vert_file),
            include_str!($frag_file),
            None).unwrap()*/
    })
}

#[macro_export]
macro_rules! shader_program_ent {
    ($display:expr, $vert_file:expr, $frag_file:expr) => ({
        // Use this for debug
        let vert_src = {
            use ::std::io::Read;
            let mut buf = String::new();
            let mut f = ::std::fs::File::open(
                format!("src/space/entities/{}", $vert_file),
            ).unwrap();
            f.read_to_string(&mut buf).unwrap();

            buf
        };

        let frag_src = {
            use ::std::io::Read;
            let mut buf = String::new();
            let mut f = ::std::fs::File::open(
                format!("src/space/entities/{}", $frag_file),
            ).unwrap();
            f.read_to_string(&mut buf).unwrap();

            buf
        };

        glium::Program::from_source($display,
                &vert_src,
                &frag_src,
                None).unwrap()
        // Use this for release
        /*glium::Program::from_source($display,
            include_str!($vert_file),
            include_str!($frag_file),
            None).unwrap()*/
    })
}
