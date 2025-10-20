/**
使用了 Rayon 库来创建一个自定义的线程池并执行并行任务。
这段代码利用 Rayon 库创建了一个 4 线程的线程池，然后使用一个 scoped task 机制将一个矩阵的 4 行数据的求和任务，分发给线程池中的线程进行 并行计算。
*/
pub fn rayon_test() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();
    let matrix = [
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
        vec![16, 17, 18, 19, 20],
    ];
    pool.scope(|scope| {
        for (i, row) in matrix.iter().enumerate() {
            scope.spawn(move |_| {
                let sum: i32 = row.iter().sum();
                println!("Sum of row {}: {}", i, sum);
            } )
        }
    } );
    println!("Main thread completed.");
}

pub fn rayon_test2() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();
    pool.scope(|scope| {
        for stage in 0..2 {
            scope.spawn(move |_scope| {
                println!("Thread {stage} started.");
                let inner_pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(2)
                    .build()
                    .unwrap();
                inner_pool.scope(|_inner_scope| {
                    for task in 0..2 {
                        _inner_scope.spawn(move |_| {
                            println!("\t-> Inner thread {stage}-{task} started.");
                        } );
                    }
                } );
                println!("\t-> Thread {stage} completed.");
            } );
        }
    } );
    println!("-> All thread completed.");
}