use criterion::{black_box, criterion_group, criterion_main, Criterion};

use gul_lang::backend::vm::VM;
use gul_lang::domains::dataflow::ir::{IRGraph, IRNode, IRPort, IREdge};
use gul_lang::frontend::ast::{Ownership, Type};

fn benchmark_vm_execution(c: &mut Criterion) {
    let mut vm = VM::new();
    let mut graph = IRGraph::new();

    let mut node1 = IRNode::new(0, "input");
    // we need lots of ports to make sure we're testing the issue
    for i in 0..100 {
        node1 = node1.with_output(IRPort::new(&format!("out{}", i), Type::Int, Ownership::Own));
    }
    let id1 = graph.add_node(node1);

    let mut node2 = IRNode::new(1, "print");
    for i in 0..100 {
        node2 = node2.with_input(IRPort::new(&format!("in{}", i), Type::Int, Ownership::Ref));
    }
    let id2 = graph.add_node(node2);

    for i in 0..100 {
        graph.add_edge(IREdge::new(id1, &format!("out{}", i), id2, &format!("in{}", i), Ownership::Ref));
    }

    graph.entry_node = Some(id1);
    graph.exit_nodes.push(id2);

    c.bench_function("vm_execution_100", |b| {
        b.iter(|| {
            black_box(vm.execute(&graph).unwrap());
        });
    });
}

criterion_group!(benches, benchmark_vm_execution);
criterion_main!(benches);
