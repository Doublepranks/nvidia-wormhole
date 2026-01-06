# PRD: Nvidia-Wormhole (v2.0)

## 1. Visão Geral
Aplicação para Linux (foco em Bazzite/Wayland) que gerencia curvas de ventoinha de GPUs NVIDIA. Soluciona o problema de "Zero RPM" em climas quentes onde o driver oficial não permite controle via Wayland.

## 2. Arquitetura "Set-and-Forget"
O software opera em dois modos separados:

### GUI Mode (Configuração)
- Interface gráfica para configurar a curva de ventoinha
- **Fecha normalmente** (não minimiza para bandeja)
- Auto-salva configuração em JSON ao ajustar sliders
- Mostra aviso se permissões não estão configuradas

### Daemon Mode (`--daemon`)
- Roda em background sem interface gráfica
- Lê configuração salva e aplica curva de ventoinha
- Iniciado automaticamente via systemd ou autostart

> **Decisão de Design:** System Tray foi descartado devido a complexidade de compatibilidade entre GTK/Qt/Wayland/X11 em diferentes DEs (especialmente KDE Plasma). A arquitetura daemon é mais robusta e "Unix-like".

## 3. Requisitos Funcionais

### Controle de Hardware
- Ler temperatura (GPU Temp) via `nvidia-settings -q`
- Definir velocidade (Fan Speed %) via `sudo nvidia-settings -a`
- Interpolar valores intermediários linearmente

### Persistência
- Config em `~/.config/nvidia-wormhole/config.json`
- Estrutura: curva de pontos (temp, speed) + intervalo do daemon

### Setup Automatizado
- **Verificação:** Testar se `sudo -n nvidia-settings --version` funciona
- **Botão "Install Service Requirements":** Usa `pkexec` para criar regra sudoers
- **Checkbox "Start daemon on login":** Cria/remove `.desktop` em `~/.config/autostart/`

### Social
- Rodapé com links para GitHub, Twitter/X, e Donate (PayPal)

## 4. Requisitos de Instalação

### Sudoers (via GUI)
```bash
# Criado automaticamente via pkexec:
/etc/sudoers.d/nvidia-wormhole
# Conteúdo: USER ALL=(ALL) NOPASSWD: /usr/bin/nvidia-settings
```

### Systemd Service (alternativa)
```bash
~/.config/systemd/user/nvidia-wormhole.service
```

### Autostart (via GUI checkbox)
```bash
~/.config/autostart/nvidia-wormhole.desktop
```

## 5. Empacotamento (Flatpak - Futuro)
* **Manifesto:** `com.github.username.nvidia-wormhole.yml`
* **Permissões Críticas:**
    * `--socket=x11` (Para GUI e acesso ao driver X11)
    * `--share=ipc` (Comunicação entre processos)
    * `--device=all` (Acesso aos devices da GPU)
    * `--talk-name=org.freedesktop.Flatpak` (Para usar o flatpak-spawn)
* **Nota:** Comandos nvidia-settings devem usar `flatpak-spawn --host`

## 6. Funcionalidades Descartadas ❌

| Feature | Motivo |
|---------|--------|
| System Tray | Incompatibilidade KDE Plasma/Wayland com libappindicator |
| Minimize to Tray | Substituído por daemon mode |
| `--minimized` flag | Substituído por `--daemon` |
| Telemetria | Não implementado nesta versão |