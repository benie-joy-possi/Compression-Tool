# Use Node.js LTS as base image
FROM node:lts

# Set working directory
WORKDIR /app

# Copy package.json and install dependencies
COPY package*.json ./
RUN npm install

# Copy rest of the app
COPY . .

# Default command (can be overridden)
CMD [ "node", "index.js" ]
 