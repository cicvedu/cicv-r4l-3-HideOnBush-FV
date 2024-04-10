// SPDX-License-Identifier: GPL-2.0

//! Rust for completion
use core::result::Result::Err;

use kernel::prelude::*;
use kernel::sync::{Mutex, CondVar};
use kernel::{chrdev, file};

const GLOBALMEM_SIZE: usize = 0x64;
module! {
    type: RustCompletion,
    name: "completion",
    author: "FV",
    description: "Rust version of 002_completion",
    license: "GPL",
}
static GLOBALMEM_BUF: Mutex<[u8;GLOBALMEM_SIZE]> = unsafe {
    Mutex::new([0u8;GLOBALMEM_SIZE])
};
static GLOBAL_CV: CondVar = unsafe {
    CondVar::new()
};

struct RustCompletion {
    _dev: Pin<Box<chrdev::Registration<1>>>,
    // data: RustFile
}


impl kernel::Module for RustCompletion {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust completion (init): {name}\n");

        let mut chrdev_reg = chrdev::Registration::new_pinned(name, 0, module)?;

        chrdev_reg.as_mut().register::<RustFile>()?;

        // let mut data = Pin::from(Box::try_new(unsafe { Mutex::new(Vec::new()) })?);
        // mutex_init!(data.as_mut(), "RustSync::init::data1");
        // let mut cv = Pin::from(Box::try_new(unsafe { CondVar::new() })?);
        // condvar_init!(cv.as_mut(), "RustSync::init::cv1");
        // let dev = Ref::try_new(RustFile {
        //     mutex: data,
        //     condvar: cv
        // })?;

        Ok(Self {
            _dev: chrdev_reg,
            // data: RustFile {
            //     mutex: data,
            //     condvar: cv
            // }
        })
    }
}

impl Drop for RustCompletion {
    fn drop(&mut self) {
        pr_info!("Rust completion (exit)\n");
    }
}

struct RustFile {
    #[allow(dead_code)]
    mutex: &'static Mutex<[u8;GLOBALMEM_SIZE]>,
    // mutex: Pin<Box<Mutex<Vec<u8>>>>,
    // condvar: Pin<Box<CondVar>>
    condvar: &'static CondVar
}

#[vtable]
impl file::Operations for RustFile {
    // type Data = Ref<'a, RustFile>;
    // type OpenData = Ref<'a, RustFile>;
    type Data = Box<Self>;

    fn open(_shared: &(), _file: &file::File) -> Result<Box<Self>> {
    // fn open(_shared: &Ref<'a, RustFile>, _file: &file::File) -> Result<Ref<'a, RustFile>> {
        pr_info!("open in chrdev");
        // Ok(_shared.clone())
        Ok(Box::try_new(RustFile {
            mutex: &GLOBALMEM_BUF,
            condvar: &GLOBAL_CV
        })?)
    }

    fn write(_this: &Self,_file: &file::File,_reader: &mut impl kernel::io_buffer::IoBufferReader,_offset:u64,) -> Result<usize> {
        pr_info!("write in rust_completion:\n");
        // let copy = _reader.read_all()?;
        // let len = copy.len();
        // let mut x = _this.inner.lock();
        // for i in 0..len {
        //     x[i] = copy[i];
        // }
        // *_this.inner.lock() = copy.try_into().unwrap();
        // pr_info!("content: {:?}", &x[0..10]);
        // Ok(len)

        // pr_info("process %d(%s) awakening the readers...\n",
        //     current->pid, current->comm);
        _this.condvar.notify_all();
        Ok(_offset as usize)
    }

    fn read(_this: &Self,_file: &file::File,_writer: &mut impl kernel::io_buffer::IoBufferWriter,_offset:u64,) -> Result<usize> {
        pr_info!("read in rust_completion\n");
    
        // pr_info("process %d(%s) is going to sleep\n", current->pid, current->comm);
        let mut guard = _this.mutex.lock();
        guard[0] = '1' as u8;
        pr_info!("get guard: {:?}\n", *guard);
        let _ = _this.condvar.wait(&mut guard);
        pr_info!("after wait\n");
        // let len = core::cmp::min(_writer.len(), x.len().saturating_sub(_offset as usize));
        // _writer.write_slice(&x[_offset as usize..][..len])?;
        // pr_info("awoken %d(%s)\n", current->pid, current->comm);
        Ok(0)
    }
}

