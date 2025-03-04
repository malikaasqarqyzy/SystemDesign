# Global Student Portal Dependency Analysis

## Architecture Stability Metrics

This repository contains:
1. Neo4j database setup for component dependencies
2. Stability metric calculations based on Clean Architecture principles

### Key Metrics
- **Fan-in**: Number of incoming dependencies
- **Fan-out**: Number of outgoing dependencies
- **Instability**: Ratio of Fan-out to total dependencies (I = Fan-out/(Fan-in + Fan-out))

### Setup
1. Start database: `docker-compose up -d`
2. Access Neo4j Browser: `http://localhost:7474`
3. Run migrations: `cat migrations/*.cypher | cypher-shell -u neo4j -p studentportal`

### Usage
1. Execute metric calculation query in Neo4j Browser
2. Results show component stability (0 = stable, 1 = unstable)

### Interpretation
- **Core Services** (I < 0.5): Authentication, File Storage
- **Unstable Components** (I = 1): Frontend, Analytics, Monitoring
- **Balanced Components**: API Gateway (I = 0.5)
