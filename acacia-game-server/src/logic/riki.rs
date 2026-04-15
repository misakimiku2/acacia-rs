pub fn get_task(n_id: u32) -> (u32, u32) {
    let n_index = n_id / 1000;
    let n_val = n_id % 1000;

    let n_task_id = (n_index - 1) * 30 + (n_val + 29) / 30;
    let n_bit = n_val % 30;

    (n_task_id, n_bit)
}
