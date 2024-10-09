# ECE 522 Project 1

## 0. Group Members: 
| Name      | ID      |
| ------------- | ------------- |
| Yicheng Yang | 1647241 |
| Jiale Cai | Cell 1, Row 2 |
| Zhouyiyang Yang | Cell 2, Row 1 |
| Yulong Zhang | 1823084 |
___

## 1. Dependencies
#### 1.1. Core dependencies
- **yahoo_finance_api**: This is the core library of the project. This library allows you to call the Yahoo Finance API and quickly query and obtain necessary data. Additionally, compared to other Yahoo Finance libraries, this library offers the latest API and has been maintained by contributors up until recently, which is why this library was chosen.
#### 1.2. Side dependencies
- **clap**: This is a library for parsing command-line arguments. It not only allows for easy retrieval of parameters but also provides a quick way to generate help documentation for the --help command.
- **tokio-test**: This is a library of testing utilities supported by **tokio**. The main purpose of importing this library is to support the operation of **yahoo_finance_api**.
---
## 2.
---
## 3.
---
## 4. Project Setup
1. To run this project, you first need to ensure that **Rust** is installed on your computer. You can visit the following website and follow the tutorial to install **Rust**:
	- https://www.rust-lang.org/learn/get-started
2. Secondly, please clone the project code from the following URL using Git or download it directly as a ZIP file:
	- https://github.com/Yulong0425/ECE_522_Proj1
3. Navigate(**cd**) to the working directory via the command line and enter: ***cargo update***, to fetch the required dependencies.
---
## 5. Usage Instructions
Navigate(**cd**) to the working directory via the command line and enter: ```cargo run -- -n (stock symbol)```, to run the project. **(stock symbol)** represents the name of the stock you want to query. For example, the following command will retrieve stock ticker data of *Apple Inc.*:```cargo run -- -n AAPL```

Additionally, after building with: ```cargo build```, you can find the compiled program in the target folder. You can also run the program directly using the following command: ```./project -n AAPL```

For a more detailed description, please run with ```--help``` as an argument