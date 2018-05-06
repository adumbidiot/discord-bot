FROM hypriot/rpi-node:latest
RUN apt-get update
RUN apt-get upgrade
RUN apt-get install git libav-tools

ADD ./ ./
RUN npm install

CMD node index.js