static mut COUNTER: u32 = 0;

#[ic_cdk::update]
fn increment() -> u32 {
    unsafe {
        COUNTER +=1;
        COUNTER
    }
}

#[ic_cdk::update]
fn decrement() -> u32 {
    unsafe {
        if COUNTER == 0 {
            return 0;
        }
        COUNTER -=1;
        COUNTER
    }
}

#[ic_cdk::query] 
fn get_count() -> u32 {
    unsafe { COUNTER }
}