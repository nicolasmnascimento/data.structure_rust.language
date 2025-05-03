use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Produto {
    id: u32,
    nome: String,
    descricao: String,
    preco: f64,
}

struct SistemaDeBusca {
    produtos: HashMap<u32, Produto>, // Armazena os produtos por ID
    indice: HashMap<String, HashSet<u32>>, // Mapeia palavras-chave para IDs de produtos
}

impl SistemaDeBusca {
    fn novo() -> Self {
        SistemaDeBusca {
            produtos: HashMap::new(),
            indice: HashMap::new(),
        }
    }

    fn adicionar_produto(&mut self, produto: Produto) {
        let id = produto.id;

        // Adiciona o produto ao HashMap
        self.produtos.insert(id, produto.clone());

        // Indexa palavras-chave do nome e da descrição
        for palavra in produto.nome.split_whitespace().chain(produto.descricao.split_whitespace()) {
            let palavra = palavra.to_lowercase();
            self.indice
                .entry(palavra)
                .or_insert_with(HashSet::new)
                .insert(id);
        }
    }

    fn buscar(&self, termo: &str) -> Vec<&Produto> {
        let termo = termo.to_lowercase();
        if let Some(ids) = self.indice.get(&termo) {
            ids.iter()
                .filter_map(|id| self.produtos.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    fn buscar_por_id(&self, id: u32) -> Option<&Produto> {
        self.produtos.get(&id)
    }

    fn listar_todos_produtos(&self) -> Vec<&Produto> {
        self.produtos.values().collect()
    }
}

fn main() {
    let mut sistema = SistemaDeBusca::novo();

    // Adicionando produtos ao sistema
    sistema.adicionar_produto(Produto {
        id: 1,
        nome: "Celular".to_string(),
        descricao: "Um smartphone moderno com câmera de alta qualidade.".to_string(),
        preco: 1500.00,
    });

    sistema.adicionar_produto(Produto {
        id: 2,
        nome: "Notebook".to_string(),
        descricao: "Notebook potente para trabalho e estudo.".to_string(),
        preco: 3500.00,
    });

    sistema.adicionar_produto(Produto {
        id: 3,
        nome: "Fone de Ouvido".to_string(),
        descricao: "Fone de ouvido com cancelamento de ruído.".to_string(),
        preco: 200.00,
    });

    // Visualizando todos os produtos cadastrados
    println!("Todos os produtos cadastrados:");
    for produto in sistema.listar_todos_produtos() {
        println!(
            "ID: {}, Nome: {}, Descrição: {}, Preço: R${:.2}",
            produto.id, produto.nome, produto.descricao, produto.preco
        );
    }
}