package de.scharschbot.velocity.plugin

import com.google.inject.Inject
import com.velocitypowered.api.event.Subscribe
import com.velocitypowered.api.event.proxy.ProxyInitializeEvent
import com.velocitypowered.api.plugin.Plugin
import com.velocitypowered.api.proxy.ProxyServer
import org.slf4j.Logger


@Plugin(id = "scharschbot", name = "ScharschBotVelocity", version = "0.1.0-SNAPSHOT", description = "Scharsch bot plugin for Velocity", authors = ["Sharktheone"])
class Plugin {
    private lateinit var server: ProxyServer
    private lateinit var logger: Logger

    @Inject
    fun scharschBot(server: ProxyServer, logger: Logger){
        this.server = server
        this.logger = logger


        logger.info("ScharschBot Velocity Plugin Loaded!")
    }

    @Subscribe
    fun onInitialize(event: ProxyInitializeEvent) {
        server.eventManager.register(this, Events(logger))
    }
}
