Logger for dakv

-----
[![Build Status](https://travis-ci.com/dakv/logger.svg?branch=master)](https://travis-ci.com/dakv/logger)
[![Version](https://img.shields.io/crates/v/dakv_logger.svg)](https://crates.io/crates/dakv_logger)
[![Coverage Status](https://coveralls.io/repos/github/dakv/logger/badge.svg?branch=master)](https://coveralls.io/github/dakv/logger?branch=master)

```Rust
use dakv_logger::prelude::*;
use dakv_logger::set_logger_level;

fn main(){
    let _logger = set_logger_level(true, None);
    info!("test");
}

```