# PickOne: Real Time Voting System

[![Cloudflare](https://img.shields.io/badge/Cloudflare-F38020?logo=Cloudflare&logoColor=white)](#built-with)
[![Docker](https://img.shields.io/badge/Docker-2496ED?logo=docker&logoColor=fff)](#built-with)
[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white)](#built-with)
[![Svelte](https://img.shields.io/badge/Svelte-%23f1413d.svg?logo=svelte&logoColor=white)](#built-with)
[![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=fff)](#built-with)
[![Figma](https://img.shields.io/badge/Figma-F24E1E?logo=figma&logoColor=white)](#built-with)

PickOne is a real-time voting system.

**Try it live:** [pickone.cc](https://pickone.cc/)

<img src="demos/live.gif" width=600 alt="GIF demonstrating live voting website">

## Table of Contents

- [Features](#features)
- [Built With](#built-with)
- [Additional Demos](#additional-demos)
- [Getting Started](#getting-started)

## Features

### Real Time

- Concurrent Users
- Live Broadcasts to All

### Frontend

- Live Bar Chart
- Click Animations
- Button Animations
- Mobile Resolutions

### Websockets

- Timeouts
- Automatic Reconnection
- Sanitization of Messages

## Built With

[![Debian](https://img.shields.io/badge/Debian-A81D33?logo=debian&logoColor=fff)](https://www.debian.org/)
[![Docker](https://img.shields.io/badge/Docker-2496ED?logo=docker&logoColor=fff)](https://www.docker.com/)  
Deployed on multiple Debian nodes using Docker Swarm

[![Cloudflare](https://img.shields.io/badge/Cloudflare-F38020?logo=Cloudflare&logoColor=white)](https://www.cloudflare.com/)
[<img src="demos/badge/caddy.png" alt="Caddy" height="21" />](https://caddyserver.com)  
Hosted using reverse proxy by Caddy and Cloudflare

[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white)](https://www.rust-lang.org/)  
Backend using Rust

[![Figma](https://img.shields.io/badge/Figma-F24E1E?logo=figma&logoColor=white)](https://www.figma.com/)
[![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=fff)](https://www.typescriptlang.org/)
[![Svelte](https://img.shields.io/badge/Svelte-%23f1413d.svg?logo=svelte&logoColor=white)](https://svelte.dev/)  
Frontend using [Figma](https://www.figma.com/design/3TCMv4E68enOcQ3quqRtO4/pickone?node-id=0-1&t=GrwhKBXnhd69lmop-1) and Typescript for Svelte

[<img src="https://img.shields.io/badge/Grafana-F46800?style=for-the-badge&logo=grafana&logoColor=white" alt="Grafana" height="21" />](https://grafana.com/) [<img src="https://img.shields.io/badge/Prometheus-E6522C?style=for-the-badge&logo=prometheus&logoColor=white" alt="Prometheus" height="21" />](https://prometheus.io/) [<img src="demos/badge/goaccess.png" width="100" height="21" alt="GoAccess"/>](https://goaccess.io/)  
Devops using Grafana (GUI, Dashboards, Loki Logging), Prometheus (Metrics, Uptime), GoAccess (Web Stats)

## Additional Demos

[<img src="demos/badge/goaccess.png" width="100" height="21" alt="GoAccess"/>](https://goaccess.io/)  
Screenshots showing examples statistics provided by GoAccess including overview stats, unique visitors, and requested links.  
<img src="demos/goaccess.png" width=600 alt="GoAccess overview of stats">
<img src="demos/goaccess_2.png" width=600 alt="GoAccess unique visitors over time">
<img src="demos/goaccess_3.png" width=600 alt="GoAccess requested links">

[<img src="https://img.shields.io/badge/Grafana-F46800?style=for-the-badge&logo=grafana&logoColor=white" alt="Grafana" height="21" />](https://grafana.com/) [<img src="https://img.shields.io/badge/Prometheus-E6522C?style=for-the-badge&logo=prometheus&logoColor=white" alt="Prometheus" height="21" />](https://prometheus.io/)  
The Grafana dashboard visualizes stats including the uptime, total users ever, number of concurrent users, and number of votes for each category for the past 24 hours.  
<img src="demos/monitor.png" width=600 alt="Grafana dashboard">

## Getting Started

### Requirements

Before running this project locally, make sure you have the following installed:

- [Git](https://git-scm.com/downloads)
- [Docker](https://docs.docker.com/engine/install/)

### Local Deployment

1. **Clone the repository**

   ```bash
   git clone https://github.com/dadal00/PickOne.git
   cd PickOne
   ```

2. **Load environment**

   ```bash
   set -a
   source .env.local
   ```

3. **Build the docker images**

   ```bash
   docker compose -f deploy/docker-build.main.yml build
   ```

4. **Start the swarm**

   ```bash
   docker swarm init
   ```

5. **Create backend hash salt**

   ```bash
   echo "your-own-salt" | docker secret create RUST_HASH_SALT -
   ```

6. **Create monitoring network**

   ```bash
   docker network create \
   --driver overlay \
   --attachable \
   --internal \
   --opt encrypted \
   monitor_net
   ```

7. **Create necessary files**

   ```bash
   touch deploy/caddy/logs/access.log
   touch deploy/saved_state.json
   touch monitor/goaccess/www/report.html
   ```

8. **Deploy the monitoring stack**

   ```bash
   docker stack deploy -c monitor/docker-swarm.monitor.local.yml monitor
   ```

9. **Deploy the main app stack**

   ```bash
   docker stack deploy -c deploy/docker-swarm.main.local.yml counter
   ```

10. **Visit the local websites**

- [Local PickOne](https://pickone/)
- [Local Grafana](http://localhost:3000/)
- GoAccess Report: Paste absolute path of `monitor/goaccess/www/report.html` into browser to view live
  - Example Absolute Path: `/Users/dadal00/Documents/PickOne/monitor/goaccess/www/report.html`
