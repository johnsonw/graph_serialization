use serde; // 1.0.120
use serde_json; // 1.0.61
use petgraph::graph; // 0.5.1
use petgraph; // 0.5.1

//                                   ┌───────────┐                  
//                                   │Root       │                  
//                                   │           │                  
//                                   └───────────┘                  
//                                         │                        
//                     ┌───────────────────┴─────────────────┐      
//                     │                                     │      
//                     ▼                                     ▼      
//               ┌───────────┐                         ┌───────────┐
//               │Component A│                         │Component B│
//               │State 1    │                         │State 1    │
//               └───────────┘                         └───────────┘
//                     │                                     │      
//                     │                                     │      
//                     ▼                                     │      
//               ┌───────────┐                               ▼      
//               │Component A│                         ┌───────────┐
//               │State 2    │                         │Component B│
//               └───────────┘                         │State 2    │
//                     │                               └───────────┘
//       ┌─────────────┴──────────────┐                             
//       ▼                            ▼                             
// ┌───────────┐                ┌───────────┐                       
// │Component C│                │Component A│                       
// │State 1    │                │State 3    │                       
// └───────────┘                └───────────┘                       

fn main () {
    #[derive(Default, serde::Serialize, serde::Deserialize)]
    struct A {
        id: i32,
        name: String,
        state: String, // State 1, State 2, State 3
        val: i32,
    }
    
    #[derive(Default, serde::Serialize, serde::Deserialize)]
    struct B {
        id: i32,
        name: String,
        state: String, // State 1, State 2
        val: String,
    }
    
    #[derive(Default, serde::Serialize, serde::Deserialize)]
    struct C {
        id: i32,
        name: String,
        state: String, // State 1
        val: i32,
    }
    
    let a1 = A {
        id: 1,
        name: "component a".into(),
        state: "State 1".into(),
        val: 1
    };
    
    let a2 = A {
        id: 2,
        name: "component a".into(),
        state: "State 2".into(),
        val: 3
    };
    
    let a3 = A {
        id: 3,
        name: "component a".into(),
        state: "State 3".into(),
        val: 7
    };
    
    let b1 = B {
        id: 1,
        name: "component b".into(),
        state: "State 1".into(),
        val: "table".into()
    };
    
    let b2 = B {
        id: 2,
        name: "component b".into(),
        state: "State 2".into(),
        val: "chair".into()
    };
    
    let c1 = C {
        id: 1,
        name: "component c".into(),
        state: "State 1".into(),
        val: 99,
    };
    
    #[derive(serde::Serialize, serde::Deserialize)]
    enum Component {
        ComponentA(A),
        ComponentB(B),
        ComponentC(C),
    }
    
    #[derive(serde::Serialize, serde::Deserialize)]
    struct PlanNode {
        component: Component
    }
    
    #[derive(serde::Serialize, serde::Deserialize)]
    struct Transition {
        name: String,
        //transition_fn: TransitionFn,
    }
    
    let mut command_plan = graph::Graph::<PlanNode, Transition>::new();
    let node_a1 = command_plan.add_node(PlanNode { component: Component::ComponentA(a1)});
    let node_a2 = command_plan.add_node(PlanNode { component: Component::ComponentA(a2)});
    let node_a3 = command_plan.add_node(PlanNode { component: Component::ComponentA(a3)});
    let node_b1 = command_plan.add_node(PlanNode { component: Component::ComponentB(b1)});
    let node_b2 = command_plan.add_node(PlanNode { component: Component::ComponentB(b2)});
    let node_c1 = command_plan.add_node(PlanNode { component: Component::ComponentC(c1)});
    
    command_plan.add_edge(node_a1, node_a2, Transition { name: "A1 to A2".into() });
    command_plan.add_edge(node_a2, node_c1, Transition { name: "A2 to C1".into() });
    command_plan.add_edge(node_a2, node_a3, Transition { name: "A2 to A3".into() });
    command_plan.add_edge(node_b1, node_b2, Transition { name: "B1 to B2".into() });
    
    
    let serialized_command_plan = serde_json::to_string(&command_plan).expect("serialize graph");
    
    println!("serialized command plan: {:?}", serialized_command_plan);
    
}

