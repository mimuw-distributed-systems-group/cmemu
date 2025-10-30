//! Facilities for interaction with cmemu hosting

use clap::Args;
use cmemu_lib::common::{
    Address,
    cmemu_hosting::{
        OS_DATA_ABI_VER, OS_DATA_ARGC, OS_DATA_ARGV, OS_DATA_ARRAYS, OS_DATA_ENVIRON, OS_DATA_RANGE,
    },
};
use cmemu_lib::engine::Emulator;
use log::{debug, info, trace};
use std::ffi::CString;

/// Pass information to the emulated application through the semihosting interface.
/// If no argument is passed, the data will not be initialized;
/// however, semihosting is still available.
#[derive(Args, Debug, Default)]
#[non_exhaustive]
pub struct HostingArgs {
    /// Pass the whole emulator environment to the emulated application.
    ///
    /// Similar to the ``-E`` (``--preserve-env``) parameter of ``sudo``.
    #[arg(long)]
    pub inherit_env: bool,

    /// Add an environment parameter to the emulated application. Use NAME=VALUE format.
    #[arg(long, short = 'E')]
    pub env: Vec<String>,

    /// Zeroth argument is traditionally the program name.
    #[arg(
        long,
        default_value = "hosted_app",
        group = "arg0_group",
        alias = "app-name"
    )]
    pub arg0: String,

    /// Don't add extra standard `arg0`.
    #[arg(long, group = "arg0_group")]
    pub no_arg0: bool,

    /// Arguments to be passed to the emulated application
    #[arg(last = true)]
    pub app_args: Vec<String>,
}

impl HostingArgs {
    /// Pass program arguments and environment to the emulated application.
    ///
    /// The data lies in a dedicated read-only memory region. With the following layout:
    ///
    /// ```text
    ///     [abi version: 0u32] [argc] [&argv: char*[][]] [&environ: char*[][]]
    ///     [argv array: char[][]]
    ///     [environ array: char[][]]
    ///     [arguments: char[]]
    ///     [environ key=value: char[]]
    /// ```
    ///
    /// Whereby ``type[]`` indicates a null-terminated array.
    /// Only the ``[abi version]`` is to be considered as a stable interface of the loader.
    /// The first line is defined in ABI version 0 as presented.
    /// The actual addresses of ``argv`` and ``environ`` are not defined.
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn process_args(&self, emulator: &mut Emulator) {
        // TODO: consider taking these args by value
        let mut args = self.app_args.clone();
        if !self.no_arg0 {
            args.insert(0, self.arg0.clone());
        }

        // TODO: environ should be a map with unique keys!
        let mut environ: Vec<String> = self.env.clone();
        if self.inherit_env {
            environ.extend(std::env::vars().map(|(k, v)| format!("{k}={v}")));
        } else if environ.is_empty() {
            // A random default env?
            environ.extend(["LC_ALL=C".to_owned(), "HOSTED=1".to_owned()]);
        }

        info!("App argc: {}, args: {args:?}", args.len());
        debug!("App environ: {environ:?}");

        // Let's start the data with a constant null at known place, so we can always make
        // an empty null-terminated array by pointing there.
        let null_addr = OS_DATA_ARRAYS;
        let null = 0u32.to_le_bytes();
        let mut argv_addr = null_addr.offset(4);
        let mut environ_addr = argv_addr.offset(4 * args.len() as u32 + 4);
        let data_start = environ_addr.offset(4 * environ.len() as u32 + 4);
        // get nicer value
        let mut data_addr = data_start.offset(8).aligned_down_to_8_bytes();

        emulator
            .write_memory(OS_DATA_ABI_VER, &null)
            .expect("Putting the ABI version failed");
        emulator
            .write_memory(
                OS_DATA_ARGC,
                &u32::try_from(args.len()).unwrap().to_le_bytes(),
            )
            .expect("Putting argc failed");
        emulator
            .write_memory(OS_DATA_ARGV, &argv_addr.to_const().to_le_bytes())
            .expect("Putting &argv failed");
        emulator
            .write_memory(OS_DATA_ENVIRON, &environ_addr.to_const().to_le_bytes())
            .expect("Putting &environ failed");

        // Put arguments
        Self::put_cstr_array(args, &mut data_addr, &mut argv_addr, emulator);
        debug_assert!(argv_addr == environ_addr);
        // Put environment
        Self::put_cstr_array(environ, &mut data_addr, &mut environ_addr, emulator);
        assert!(data_addr.is_in_range(&OS_DATA_RANGE));
    }

    #[allow(clippy::cast_possible_truncation)]
    fn put_cstr_array(
        args: Vec<impl Into<Vec<u8>>>,
        data_addr: &mut Address,
        arr_addr: &mut Address,
        emulator: &mut Emulator,
    ) {
        for arg in args {
            let arg = CString::new(arg).unwrap();
            trace!("Placing {arg:?} at {data_addr:?} with ptr at {arr_addr:?}");
            let arg = arg.as_bytes_with_nul();

            emulator
                .write_memory(*arr_addr, &data_addr.to_const().to_le_bytes())
                .expect("Putting array entry pointer failed");
            emulator
                .write_memory(*data_addr, arg)
                .expect("Putting array argument failed");

            *arr_addr = arr_addr.offset(4);
            *data_addr = data_addr.offset(arg.len() as u32);
        }
        emulator
            .write_memory(*arr_addr, &0u32.to_le_bytes())
            .expect("Putting final array entry failed");
        *arr_addr = arr_addr.offset(4);
    }
}
