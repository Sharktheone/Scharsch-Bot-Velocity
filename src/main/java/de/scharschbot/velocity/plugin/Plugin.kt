package de.scharschbot.velocity.plugin

import com.fasterxml.jackson.databind.JsonNode
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory
import com.google.inject.Inject
import com.velocitypowered.api.event.Subscribe
import com.velocitypowered.api.event.connection.DisconnectEvent
import com.velocitypowered.api.event.connection.PostLoginEvent
import com.velocitypowered.api.plugin.Plugin
import com.velocitypowered.api.proxy.ProxyServer
import org.apache.http.auth.UsernamePasswordCredentials
import org.apache.http.client.methods.CloseableHttpResponse
import org.apache.http.client.methods.HttpPost
import org.apache.http.entity.StringEntity
import org.apache.http.impl.auth.BasicScheme
import org.apache.http.impl.client.HttpClients
import org.slf4j.Logger
import java.io.File
import java.io.IOException
import java.nio.file.Files
import java.nio.file.Paths


@Plugin(id = "scharschbot", name = "ScharschBotVelocity", version = "0.1.0-SNAPSHOT", description = "Scharsch bot plugin for Velocity", authors = ["Sharktheone"])
class Plugin {
    private var server: ProxyServer? = null
    private var logger: Logger? = null
    private lateinit var config: JsonNode

    @Inject
    fun scharschBot(server: ProxyServer, logger: Logger){
        this.server = server
        this.logger = logger
        this.config = getConfig()

        logger.info("ScharschBot Velocity Plugin Loaded!")
    }

    @Subscribe
    fun onInitialize(event: ProxyInitializeEvent) {
        server.eventManager.register(this, Events(logger))
    }
}