/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#![feature(uniform_paths)]
#![cfg_attr(feature = "fatal-warnings", deny(warnings))]

extern crate criterion;
extern crate rpds;

mod utils;

use ::criterion::{black_box, Criterion, criterion_main, criterion_group};
use ::rpds::List;
use crate::utils::limit;

fn rpds_list_push_front(c: &mut Criterion) {
    let limit = limit(10_000);

    c.bench_function("rpds list push front", move |b| {
        b.iter(|| {
            let mut list: List<usize> = List::new();

            for i in 0..limit {
                list = list.push_front(i);
            }

            list
        })
    });
}

fn rpds_list_push_front_mut(c: &mut Criterion) {
    let limit = limit(10_000);

    c.bench_function("rpds list push front mut", move |b| {
        b.iter(|| {
            let mut list: List<usize> = List::new();

            for i in 0..limit {
                list.push_front_mut(i);
            }

            list
        })
    });
}

fn rpds_list_drop_first(c: &mut Criterion) {
    let limit = limit(10_000);

    c.bench_function("rpds list drop first", move |b| {
        b.iter_with_setup(
            || {
                let mut list: List<usize> = List::new();

                for i in 0..limit {
                    list.push_front_mut(i);
                }

                list
            },
            |mut list| {
                for _ in 0..limit {
                    list = list.drop_first().unwrap();
                }

                list
            },
        );
    });
}

fn rpds_list_drop_first_mut(c: &mut Criterion) {
    let limit = limit(10_000);

    c.bench_function("rpds list drop first mut", move |b| {
        b.iter_with_setup(
            || {
                let mut list: List<usize> = List::new();

                for i in 0..limit {
                    list.push_front_mut(i);
                }

                list
            },
            |mut list| {
                for _ in 0..limit {
                    list.drop_first_mut();
                }

                list
            },
        );
    });
}

fn rpds_list_reverse(c: &mut Criterion) {
    let limit = limit(1_000);

    c.bench_function("rpds list reverse", move |b| {
        b.iter_with_setup(
            || {
                let mut list: List<usize> = List::new();

                for i in 0..limit {
                    list.push_front_mut(i);
                }

                list
            },
            |mut list| {
                for _ in 0..limit {
                    list = list.reverse();
                }

                list
            },
        );
    });
}

fn rpds_list_reverse_mut(c: &mut Criterion) {
    let limit = limit(1_000);

    c.bench_function("rpds list reverse mut", move |b| {
        b.iter_with_setup(
            || {
                let mut list: List<usize> = List::new();

                for i in 0..limit {
                    list.push_front_mut(i);
                }

                list
            },
            |mut list| {
                for _ in 0..limit {
                    list.reverse_mut();
                }

                list
            },
        );
    });
}

fn rpds_list_iterate(c: &mut Criterion) {
    let limit = limit(10_000);
    let mut list: List<usize> = List::new();

    for i in 0..limit {
        list.push_front_mut(i);
    }

    c.bench_function("rpds list iterate", move |b| {
        b.iter(|| {
            for i in list.iter() {
                black_box(i);
            }
        })
    });
}

criterion_group!(
    benches,
    rpds_list_push_front,
    rpds_list_push_front_mut,
    rpds_list_drop_first,
    rpds_list_drop_first_mut,
    rpds_list_reverse,
    rpds_list_reverse_mut,
    rpds_list_iterate
);
criterion_main!(benches);
