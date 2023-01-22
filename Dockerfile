from ubuntu:focal

SHELL ["/bin/sh", "-c"]
ENV DEBIAN_FRONTEND=noninteractive

RUN apt update && apt install -y software-properties-common curl lsb-release gnupg2 && \
	add-apt-repository universe

RUN curl -sSL https://raw.githubusercontent.com/ros/rosdistro/master/ros.key -o /usr/share/keyrings/ros-archive-keyring.gpg && \
	echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/ros-archive-keyring.gpg] http://packages.ros.org/ros2/ubuntu $(. /etc/os-release && echo $UBUNTU_CODENAME) main" | tee /etc/apt/sources.list.d/ros2.list > /dev/null

RUN apt update && apt upgrade && \
	apt install -y ros-foxy-ros-base \
	python3-argcomplete && \
	apt install -y ros-foxy-rviz2

RUN useradd --create-home --home-dir /home/cdross-dir --shell /bin/bash --user-group --groups sudo cdross && \
	echo cdross:cdross | chpasswd && \
	echo "cdross ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

RUN echo "source /opt/ros/foxy/setup.sh" >> /home/cdross-dir/.bashrc && \
	echo "source /usr/share/colcon_argcomplete/hook/colcon-argcomplete.sh" >> /home/cdross-dir/.bashrc 

USER cdross
WORKDIR /home/cdross-dir
