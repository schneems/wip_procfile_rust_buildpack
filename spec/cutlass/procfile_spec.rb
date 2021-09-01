# frozen_string_literal: true

require_relative '../spec_helper'

RSpec.describe "Cloud Native Buildpack" do
  it "builds" do
    Cutlass::App.new(
      "helloworld"
    ).transaction do |app|
      app.pack_build do |result|
        puts result.stdout
      end

      payload = SecureRandom.hex(10)
      app.start_container(expose_ports: [8080]) do |container|
        response = Excon.get(
          "http://localhost:#{container.port(8080)}/?payload=#{payload}",
          idempotent: true,
          retry_limit: 5,
          retry_interval: 1
        )

        expect(response.body).to include("?payload=#{payload}")
      end
    end
  end
end
