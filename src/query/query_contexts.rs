use std::collections::HashMap;
use std::time::{SystemTime}; 

pub struct ThreadInfo {
    pub interruption_requested: bool,
    finished: bool,
    pub worker_index: u32,
    pub timeout: SystemTime,    
    time_start: SystemTime,
}

impl ThreadInfo {
    pub fn new() -> Self {
        Self {
            // Initialize your struct here
            // For example:
            interruption_requested: false,
            finished: false,
            worker_index: 0,
            timeout: SystemTime::now(),
            time_start: SystemTime::now(),
        }
    }
}


pub struct VarContext {
    internal_var_counter: u64,
    var_names: Vec<String>,
    var_map: HashMap<String, u64>,
}

impl VarContext {
    pub fn new() -> Self {
        Self {
            // Initialize your struct here
            // For example:
            internal_var_counter: 0,
            var_names: Vec::new(),
            var_map: HashMap::new(),
        }
    }
}

pub struct QueryContext {
    pub thread_info: ThreadInfo,
    blank_node_ids: HashMap<String, u64>,
    var_ctx: VarContext,
    blank_node_count: u64,
}

impl QueryContext {
    pub fn reset(&mut self) {
        self.blank_node_ids.clear();
        self.blank_node_count = 0;

        self.var_ctx.internal_var_counter = 0;
        self.var_ctx.var_names.clear();
        self.var_ctx.var_map.clear();
    }


    pub fn new() -> Self {
        Self {
            thread_info: ThreadInfo::new(), 
            blank_node_ids: HashMap::new(),
            var_ctx: VarContext::new(), 
            blank_node_count: 0,
        }
    }
}
