**Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore**

**Descrição do Projeto**

Este projeto implementa um sistema de busca otimizado para o catálogo de produtos da MegaStore. Ele utiliza tabelas hash para mapear palavras-chave diretamente aos IDs dos produtos, permitindo buscas rápidas e precisas. O sistema é projetado para ser eficiente, escalável e fácil de usar.

**Funcionalidades**
- Cadastro de Produtos: Adicione produtos com ID, nome, descrição e preço.
- Busca por Palavra-Chave: Realize buscas rápidas utilizando palavras-chave presentes no nome ou na descrição dos produtos.
- Busca por ID: Encontre um produto específico pelo seu ID.
- Listagem de Produtos: Liste todos os produtos cadastrados no sistema.

**Arquitetura do Sistema**

O sistema é composto pelos seguintes componentes principais:

**Estruturas**

_- Produto:_
  - Representa os produtos com os seguintes atributos:
    - id: Identificador único do produto.
    - nome: Nome do produto.
    - descricao: Descrição detalhada do produto.
    - preco: Preço do produto.
  
_- SistemaDeBusca:_
  - Gerencia os produtos e o índice de busca.
  - Atributos:
    - produtos: Um HashMap que armazena os produtos por ID.
    - indice: Um HashMap que mapeia palavras-chave para conjuntos de IDs de produtos.

_- Métodos_
  - novo: Cria uma nova instância do sistema de busca.
  - adicionar_produto: Adiciona um produto ao sistema e indexa suas palavras-chave.
  - buscar: Realiza buscas por palavras-chave.
  - buscar_por_id: Busca produtos específicos pelo ID.
  - listar_todos_produtos: Lista todos os produtos cadastrados.

**Algoritmos e Estruturas de Dados Utilizados**

_- Tabelas Hash (HashMap):_
  - Utilizadas para armazenar os produtos por ID e para mapear palavras-chave aos IDs dos produtos.
    
_- Conjuntos (HashSet):_
  - Utilizados para gerenciar os IDs associados a cada palavra-chave.
    
_- Indexação de Palavras-Chave:_
  - As palavras do nome e da descrição de cada produto são extraídas e armazenadas no índice para buscas rápidas.
