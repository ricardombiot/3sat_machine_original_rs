use crate::abssat::gpath::visual::PathDiagram;
use crate::abssat::gpath::visual::GPow;
use crate::abssat::utils::alias::{PathNodeId, Step, path_id_as_key};

impl<'a> PathDiagram<'a> {

    pub fn build(graph : &'a GPow) -> PathDiagram {
        let mut diagram = PathDiagram::new(graph);
        diagram.build_diagram();

        return diagram;
    }

    pub fn build_diagram(&mut self){
        self.dot_txt += "digraph G {\n";
        self.dot_txt += "     compound=true \n";
        self.draw();
        self.draw_relations();
        self.dot_txt += "}";
    }

    fn draw(&mut self){
        for step in 0..self.graph.get_current_step(){
            self.draw_line(step);
        }
    }

    fn draw_line(&mut self, step : Step){
        self.dot_txt += &format!("subgraph cluster_line_{} {{\n", step).to_owned();
        self.dot_txt += " style=filled;\n";
        self.dot_txt += " color=lightgrey; \n";
        self.dot_txt += "     node [style=filled,color=white]; \n";
        for node_id in self.graph.get_set_line(step) {
            self.draw_node(node_id);
        }
        self.dot_txt += "\n";
        self.dot_txt += "     fontsize=\"12\" \n";
        self.dot_txt += &format!("     label = \"Line {} \" \n", step).to_owned();
        self.dot_txt += " }\n";
    }

    fn draw_node(&mut self, path_id_node : PathNodeId){
        let key = path_id_as_key(path_id_node);
        let title = "";
        //let mut node_label_html = String::new();
        let node_label_html = format!("<{}<BR /> ID: {} <BR />>", title, key);
        /*#node_label_html *= "Using: [$list_using_me]<BR />"

        #node_label_html *= "Requires: [$list_requires] <BR />"
        #node_label_html *= "<BR /><FONT POINT-SIZE=\"8\">Parents: $parents_nodes_txt</FONT>"
        #node_label_html *= "<BR /><FONT POINT-SIZE=\"8\">Sons: $sons_nodes_txt</FONT>"
        #node_label_html *= owners_html

        #node_label_html *= draw_owners(node)*/
        //node_label_html += ">"
        self.dot_txt += &format!("{} [label={}]", key, node_label_html).to_owned();
    }

    fn draw_relations(&mut self){
        for step in 0..self.graph.get_current_step() {
            for path_id_node in self.graph.get_set_line(step) {
                let key_origin = path_id_as_key(path_id_node);
                for node_id_son in self.graph.get_node_sons_owners(path_id_node) {
                    let key_destine = path_id_as_key(node_id_son);

                    self.dot_txt += &format!("{} -> {}", key_origin, key_destine).to_owned();
                }
            }
        }
    }
}