# -*- mode: ruby -*-
# vi: set ft=ruby ts=2 sw=2 expandtab :

PROJECT = "rust-breakout"
PROJECT_DIRECTORY = "/home/vagrant/#{PROJECT}"

DOCKER_ENV = {
  "HOST_USER_UID" => Process.euid,
  "PROJECT_DIRECTORY" => "#{PROJECT_DIRECTORY}",
  "APP_PATH" => "#{PROJECT_DIRECTORY}/rust-breakout",
}

ENV['VAGRANT_NO_PARALLEL'] = 'yes'
ENV['VAGRANT_DEFAULT_PROVIDER'] = 'docker'
Vagrant.configure(2) do |config|

  config.ssh.insert_key = false
  config.vm.define "dev", primary: true do |app|
    app.vm.provider "docker" do |d|
      d.image = "allansimon/docker-dev-rust"
      d.name = "#{PROJECT}_dev"
      d.has_ssh = true
      d.env = DOCKER_ENV
    end
    app.ssh.username = "vagrant"
  end
end
