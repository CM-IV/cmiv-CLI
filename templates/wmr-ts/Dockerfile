#Dockerfile
FROM node:14-slim

WORKDIR /app

COPY ./package*.json ./

RUN npm install

COPY . .

#This port coincides with what the dev server runs on
EXPOSE 3000

CMD ["npm", "run", "start"]