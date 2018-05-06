FROM hypriot/rpi-node:latest
RUN apt-get update
RUN apt-get upgrade

ADD ./ ./
RUN npm install

CMD node index.js