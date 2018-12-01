///struct for UndirectedGraph.
pub struct UndirectedGraph<VP: Property,EP: Property> {
    n : usize,
    m : usize,
    g : Vec<Vec<Edge>>,
    es : Vec<EP>,
    vs : Vec<VP>
}

use graph::*;
use graph::property::Property;

impl<'a,VP : Property ,EP : Property> Graph<'a,VP,EP> for UndirectedGraph<VP,EP> {
    type EIter = std::slice::Iter<'a,Edge>;
    
    fn add_edge(&mut self , from : &Vertex , to : &Vertex , edge_prop : EP) {
        self.g[from.0].push(Edge{index : self.m , from : from.clone() , to : to.clone()});
        self.g[to.0].push(Edge{index : self.m, from : to.clone(), to : from.clone()});
        self.es.push(edge_prop);
        self.m += 1;
    }
    fn vertices_cnt(&self) -> usize { self.n }
    fn edges_cnt(&self) -> usize { self.m }
    fn vprop_mut(&mut self, v : &Vertex) -> &mut VP {
        &mut self.vs[v.0]
    }
    fn vprop(&self, v : &Vertex) -> &VP {
        & self.vs[v.0]
    }
    fn eprop_mut(&mut self, e : &Edge) -> &mut EP {
        &mut self.es[e.index]
    }
    fn eprop(&self, e : &Edge) -> &EP {
        & self.es[e.index]
    }

    fn delta(&'a self , v : &Vertex) -> Self::EIter {
        self.g[v.0].iter()
    }
}

impl<'a,VP : Property ,EP : Property> StaticGraph<'a,VP,EP> for UndirectedGraph<VP,EP> {
    fn new(n : usize , vp_init: VP) -> Self {
        UndirectedGraph {
            n: n,
            m: 0,
            g: vec![Vec::<Edge>::new(); n],
            es: Vec::<EP>::new(),
            vs: vec![vp_init; n]
        }
    }
    
}
impl<'a,VP : Property, EP: Property> Undirected<'a,VP,EP> for UndirectedGraph<VP,EP> {
}
