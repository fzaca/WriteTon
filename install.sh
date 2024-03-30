#!/bin/bash

# Directorio de instalación
INSTALL_DIR="$HOME/.local/share/writeton"

# Binario de la app
BIN_URL="https://github.com/fzaca/WriteTon/releases/download/v0.1.0/writeton"

# Title
install() {
  echo ""
  echo " _       __     _ __     ______"
  echo "| |     / /____(_) /____/_  __/___  ____"
  echo "| | /| / / ___/ / __/ _ \/ / / __ \/ __ \ "
  echo "| |/ |/ / /  / / /_/  __/ / / /_/ / / / /"
  echo "|__/|__/_/  /_/\__/\___/_/  \____/_/ /_/"
  echo ""

  # Crea el directorio de instalación si no existe
  mkdir -p "$INSTALL_DIR"

  # Descargar binario
  echo "Downloading the application binary..."
  wget -nv -q --show-progress "$BIN_URL" -O "$INSTALL_DIR/writeton"

  # Asegurarse que sea ejecutable
  chmod +x "$INSTALL_DIR/writeton"

  echo ""
  echo "The WriteTon application has been successfully installed in $INSTALL_DIR"
  echo ""

  # Add the directory to the PATH
  echo ""
  echo "To add WriteTon to your PATH, run the following command:"
  echo "export PATH=\"$INSTALL_DIR:\$PATH\""
  echo ""
  echo "Or to make it permanent, add this line to your ~/.bashrc or ~/.bash_profile file:"
  echo "echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> ~/.bashrc"
  echo ""
  echo "Or for zsh:"
  echo "echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> ~/.zshrc"
  echo ""
}

install
