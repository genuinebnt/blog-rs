# Axum Blog

A modern, high-performance blog application built with Rust using the Axum web framework. This project demonstrates best practices for building scalable web applications with async Rust, featuring PostgreSQL integration, Docker containerization, and automated CI/CD pipelines.

## 🚀 Features

### Current Implementation
- ✅ **Health Check Endpoint** - Application health monitoring
- ✅ **Subscription System** - User email subscription functionality
- ✅ **PostgreSQL Integration** - Database connectivity with SQLx
- ✅ **Database Migrations** - Automated schema management
- ✅ **Docker Support** - Multi-stage builds with production optimizations
- ✅ **Configuration Management** - Environment-based configuration
- ✅ **CI/CD Pipeline** - Automated testing and deployment with GitHub Actions

### Planned Features
- 📝 **Blog Post Management** - Create, read, update, delete blog posts
- 👤 **User Authentication** - JWT-based authentication system
- 🏷️ **Tag System** - Categorize posts with tags
- 💬 **Comment System** - User comments on blog posts
- 🔍 **Search Functionality** - Full-text search across posts
- 📊 **Analytics Dashboard** - Post views and engagement metrics
- 🎨 **Rich Text Editor** - Markdown support with live preview
- 📱 **Responsive Design** - Mobile-friendly web interface

## 🛠️ Tech Stack

- **Backend**: Rust with Axum web framework
- **Database**: PostgreSQL with SQLx for async queries
- **Containerization**: Docker with multi-stage builds
- **CI/CD**: GitHub Actions
- **Configuration**: Environment-based with `config` crate
- **Testing**: Integration and unit tests with test database
- **Security**: Planned JWT authentication and input validation

## 📋 Prerequisites

- Rust 1.70+ (2024 edition)
- Docker and Docker Compose
- PostgreSQL 16+ (if running locally)
- Git

## 🚦 Quick Start

### 1. Clone the Repository
```bash
git clone https://github.com/your-username/axum-blog.git
cd axum-blog
```

### 2. Environment Setup
```bash
# Copy the example environment file
cp .env.example .env.dev

# Edit .env.dev with your configuration
vim .env.dev
```

### 3. Development with Docker
```bash
# Start the development environment
./scripts/deploy.sh dev up -d

# View logs
./scripts/deploy.sh dev logs -f

# Stop the environment
./scripts/deploy.sh dev down
```

### 4. Local Development
```bash
# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features native-tls,postgres

# Start PostgreSQL (using Docker)
docker run -d \
  --name blog-postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=blog \
  -p 5432:5432 \
  postgres:16

# Run migrations
sqlx migrate run

# Start the application
cargo run
```

## 🏗️ Project Structure

```
axum-blog/
├── src/
│   ├── config.rs          # Configuration management
│   ├── lib.rs             # Library root
│   ├── main.rs            # Application entry point
│   ├── startup.rs         # Application setup and routing
│   └── routes/
│       ├── mod.rs         # Route module declarations
│       ├── health_check.rs # Health check endpoint
│       └── subscribe.rs    # Subscription functionality
├── tests/
│   ├── health_check.rs    # Health check integration tests
│   ├── subscriptions.rs   # Subscription tests
│   └── utils.rs           # Test utilities
├── migrations/
│   └── *.sql              # Database migration files
├── scripts/
│   └── deploy.sh          # Deployment automation script
├── .github/workflows/     # CI/CD pipelines
├── docker-compose*.yml    # Docker orchestration
├── Dockerfile             # Container build instructions
└── Cargo.toml            # Rust project configuration
```

## 🔧 Configuration

The application uses environment-based configuration. Set these variables in your `.env` file or environment:

### Application Settings
```bash
APP_APPLICATION__HOST=localhost
APP_APPLICATION__PORT=3000
```

### Database Settings
```bash
APP_DATABASE__USERNAME=postgres
APP_DATABASE__PASSWORD=your_password
APP_DATABASE__HOST=localhost
APP_DATABASE__PORT=5432
APP_DATABASE__DATABASE_NAME=blog
APP_DATABASE__REQUIRE_SSL=false
```

### Docker Environment
```bash
POSTGRES_DB=blog
POSTGRES_USER=postgres
POSTGRES_PASSWORD=your_password
DATABASE_URL=postgresql://postgres:your_password@localhost:5432/blog
```

## 🧪 Testing

### Run All Tests
```bash
cargo test
```

### Run Specific Test Suites
```bash
# Unit tests only
cargo test --lib

# Integration tests
cargo test --test health_check
cargo test --test subscriptions
```

### Test with Database
```bash
# Ensure test database is running
export DATABASE_URL=postgresql://postgres:password@localhost:5432/blog_test

# Run migrations on test database
sqlx migrate run

# Run tests
cargo test
```

## 🐳 Docker Usage

### Development Environment
```bash
# Start services
./scripts/deploy.sh dev up -d

# View service status
./scripts/deploy.sh dev ps

# Access application shell
./scripts/deploy.sh dev shell

# Clean up resources
./scripts/deploy.sh dev clean
```

### Production Deployment
```bash
# Create production secrets
./scripts/deploy.sh secrets generate
# Edit secret files in secrets/ directory
./scripts/deploy.sh secrets create

# Deploy to production
./scripts/deploy.sh prod deploy

# Check deployment status
./scripts/deploy.sh prod status

# View production logs
./scripts/deploy.sh prod logs
```

## 🔄 CI/CD Pipeline

The project includes automated GitHub Actions workflows:

### Pull Request Checks (`pr-check.yml`)
- Code formatting validation
- Clippy linting
- Security audit
- Test suite execution
- Build verification

### Production Pipeline (`ci-cd.yml`)
- Comprehensive testing
- Security scanning
- Docker image building
- Container registry publishing
- Production deployment

## 📊 API Endpoints

### Current Endpoints

| Method | Endpoint | Description | Status |
|--------|----------|-------------|---------|
| GET | `/health_check` | Application health status | ✅ Implemented |
| POST | `/subscribe` | Email subscription | ✅ Implemented |

### Planned Endpoints

| Method | Endpoint | Description | Status |
|--------|----------|-------------|---------|
| GET | `/api/posts` | List blog posts | 📋 Planned |
| GET | `/api/posts/{id}` | Get specific post | 📋 Planned |
| POST | `/api/posts` | Create new post | 📋 Planned |
| PUT | `/api/posts/{id}` | Update existing post | 📋 Planned |
| DELETE | `/api/posts/{id}` | Delete post | 📋 Planned |
| POST | `/api/auth/login` | User authentication | 📋 Planned |
| POST | `/api/auth/register` | User registration | 📋 Planned |

## 🗃️ Database Schema

### Current Schema

```sql
-- Users table for subscriptions and authentication
CREATE TABLE USERS(
    ID UUID UNIQUE NOT NULL,
    PRIMARY KEY (ID),
    EMAIL TEXT NOT NULL UNIQUE,
    NAME TEXT UNIQUE NOT NULL,
    PASSWORD TEXT UNIQUE NOT NULL,
    CREATED_AT TIMESTAMPTZ NOT NULL
);
```

### Planned Schema Extensions

- **Posts** - Blog post content and metadata
- **Tags** - Post categorization system
- **Comments** - User comments on posts
- **Post_Tags** - Many-to-many relationship table
- **Sessions** - User session management

## 🚀 Deployment

### Local Development
1. Clone the repository
2. Set up environment variables
3. Run `cargo run` or use Docker Compose

### Production Deployment
1. Build Docker image: `docker build -t axum-blog .`
2. Deploy using provided scripts: `./scripts/deploy.sh prod deploy`
3. Monitor with: `./scripts/deploy.sh prod status`

### Environment Variables for Production
- Set up proper database credentials
- Configure SSL certificates
- Set secure JWT secrets
- Configure logging levels

## 🛡️ Security

### Current Security Measures
- Environment-based configuration
- SQL injection prevention with SQLx
- Docker security best practices
- Automated security auditing in CI

### Planned Security Features
- JWT-based authentication
- Password hashing with bcrypt
- Rate limiting
- Input validation and sanitization
- CORS configuration
- Security headers

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and add tests
4. Ensure all tests pass: `cargo test`
5. Commit your changes: `git commit -m 'Add amazing feature'`
6. Push to the branch: `git push origin feature/amazing-feature`
7. Open a Pull Request

### Development Guidelines
- Follow Rust naming conventions
- Add tests for new functionality
- Update documentation as needed
- Ensure CI pipeline passes
- Use conventional commit messages

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Modern async web framework
- [SQLx](https://github.com/launchbadge/sqlx) - Async SQL toolkit
- [Tokio](https://tokio.rs/) - Async runtime for Rust
- [Docker](https://www.docker.com/) - Containerization platform

## 📞 Support

- Create an [Issue](https://github.com/your-username/axum-blog/issues) for bug reports
- Start a [Discussion](https://github.com/your-username/axum-blog/discussions) for questions
- Check the [Wiki](https://github.com/your-username/axum-blog/wiki) for detailed documentation

---

**Made with ❤️ and Rust**
