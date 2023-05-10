# Rust Neural Network

This is a simple neural network implementation in Rust.

## Usage

To use this neural network, create a new instance of the `NeuralNetwork` struct and add layers using the `add_layer` method. Train the network using the `train` method, passing in input and target values along with a learning rate.

## Design Decisions

The neural network is designed with a modular approach, separating the functionality of the network, layers, and neurons into distinct structs. The implementation follows Rust's best practices for safety, performance, and code organization.

## Abstract

The neural network is a basic feedforward neural network with backpropagation for training. It is implemented in Rust to showcase the language's capabilities in creating a safe and performant application.
