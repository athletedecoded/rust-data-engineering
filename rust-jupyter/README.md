# Rust x Jupyter

Integrating Rust into Jupyter notebooks with EvCxR (Evaluation Context for Rust)


## Install evcxr-jupyter

Install evcxr rust kernel for jupyter according to [official docs](https://github.com/evcxr/evcxr/tree/main/evcxr_jupyter)

```
# NB: .devcontainer/Dockerfile must include `jupyter-notebooks`
# NB: .devcontainer/devcontainer.json must include jupyter vscode ext `ms-toolsai.jupyter`

cd rust-jupyter
cargo install --locked evcxr_jupyter
evcxr_jupyter --install
```

## Run notebook.ipynb

Launch `./notebook.ipynb`

## Resources

* [EvCxR Jupyter Demo](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/samples/evcxr_jupyter_tour.ipynb) 


