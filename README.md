# Sistema de Busca em Rust - Mega Store

Este projeto implementa um sistema de busca simples utilizando a linguagem de programação Rust. O sistema permite cadastrar produtos, realizar buscas por palavras-chave e listar todos os produtos cadastrados. Ele utiliza tabelas hash para mapear palavras-chave diretamente aos IDs dos produtos, garantindo eficiência e rapidez nas buscas.

## Funcionalidades

- **Cadastro de Produtos**: Adicione produtos com ID, nome, descrição e preço.
- **Busca por Palavra-Chave**: Realize buscas rápidas utilizando palavras-chave presentes no nome ou na descrição dos produtos.
- **Busca por ID**: Encontre um produto específico pelo seu ID.
- **Listagem de Produtos**: Liste todos os produtos cadastrados no sistema.

## Estrutura do Código

- **Struct `Produto`**: Representa os produtos com os seguintes atributos:
  - `id`: Identificador único do produto.
  - `nome`: Nome do produto.
  - `descricao`: Descrição detalhada do produto.
  - `preco`: Preço do produto.

- **Struct `SistemaDeBusca`**: Gerencia os produtos e o índice de busca.
  - `produtos`: Um `HashMap` que armazena os produtos por ID.
  - `indice`: Um `HashMap` que mapeia palavras-chave para conjuntos de IDs de produtos.
