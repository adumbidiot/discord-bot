FROM hypriot/rpi-node:latest
RUN apt-get update
RUN apt-get upgrade
RUN apt-get install git libav-tools

ADD ./package.json ./package.json
ADD ./package-lock.json ./package-lock.json
RUN npm install

ADD ./ ./

CMD node index.js