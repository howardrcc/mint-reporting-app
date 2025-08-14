# Build stage
FROM node:20-alpine as builder

WORKDIR /app

# Copy package files
COPY web-frontend/package*.json ./
COPY shared/ ../shared/

# Install dependencies including dev dependencies needed for build
RUN npm ci --legacy-peer-deps

# Copy source code
COPY web-frontend/ .

# Build the application
ENV NODE_ENV=production
ENV VITE_API_URL=http://localhost:3000
RUN npx vite build

# Production stage
FROM nginx:alpine

# Copy custom nginx config
COPY docker/nginx.conf /etc/nginx/nginx.conf

# Copy built application
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy site configuration
COPY docker/default.conf /etc/nginx/conf.d/default.conf

# Expose port
EXPOSE 80

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost/ || exit 1

# Start nginx
CMD ["nginx", "-g", "daemon off;"]