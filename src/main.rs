/* Spec
    指定できるべきもの：
        - aggregation
            - sum
            - cnt
            - avg
        - target
            - uri
                - can be regexp or direct path
            - method
            - status

    パースすべきもの
        - time
        - method
        - uri
        - status
        - reqtime
*/
extern crate chrono;
extern crate als;

use std::io;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let access_logs = als::read_access_log(stdin);
    println!("{}", access_logs.len());
}