FROM node:16.13.2

COPY . .

RUN npm i -c
RUN npm run build
RUN node -v
CMD ["npm", "run", "preview"]

EXPOSE 4173
