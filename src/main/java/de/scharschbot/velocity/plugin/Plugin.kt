package de.scharschbot.velocity.plugin

import com.fasterxml.jackson.databind.JsonNode
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory
import com.google.inject.Inject
import com.velocitypowered.api.event.Subscribe
import com.velocitypowered.api.event.proxy.ProxyInitializeEvent
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


    private fun getConfig(): JsonNode {
        val configName = "config.yml"
        val configPath = Paths.get("./plugins/scharschbot/$configName")

        val config = File(configPath.toString())
        if(!config.exists()){
            try {
                Files.createDirectories(Paths.get(config.parent))
            } catch (e: IOException) {
                // ignore
            }
            try {
                javaClass.classLoader.getResourceAsStream(configName).use { standardConfig ->
                    if (standardConfig != null) {
                        Files.copy(
                            standardConfig,
                            configPath
                        )
                    }
                }
            } catch (e: IOException) {
                logger?.warn("Could not copy config file!")
                throw RuntimeException(e)
            }
        }
        val mapper = ObjectMapper(YAMLFactory())


        return mapper.readTree(config)
        }

    private fun sendValues(Data: String){
        val httpClient = HttpClients.createDefault()
        try {
            val request = HttpPost(config.get("URL")?.asText())
            val creds = UsernamePasswordCredentials(config.get("User")?.asText(),config.get("Pass")?.asText())

            request.entity = StringEntity(Data)
            request.setHeader("Content-type", "application/json")
            request.addHeader(BasicScheme().authenticate(creds, request, null))

            val response: CloseableHttpResponse = httpClient.execute(request)
            if ( !(response.statusLine.statusCode == 204 || response.statusLine.statusCode == 200) ) {
                logger?.warn("Failure sending data to discord bot: " + response.statusLine.reasonPhrase)
            }
            response.close()
            httpClient.close()
        } catch (e: Exception) {
            logger?.warn("Failed to send HTTP Request: " + e.message)
        }
    }

    @Subscribe
    fun onProxyInitialization(event: ProxyInitializeEvent?) {
    }

    @Subscribe
    fun playerJoin(event: PostLoginEvent){
        val joinJson = "{\"name\":\"" + event.player.username + "\", \"type\":\"join\", \"server\":\"" + config.get("ServerName")?.asText() + "\"}"
        sendValues(joinJson)
    }

    @Subscribe
    fun playerQuit(event: DisconnectEvent){
        val quitJson = "{\"name\":\"" + event.player.username + "\", \"type\":\"quit\", \"server\":\"" + config.get("ServerName")?.asText() + "\"}"
        sendValues(quitJson)
    }
}