package de.scharschbot.velocity.plugin

import com.velocitypowered.api.event.Subscribe
import com.velocitypowered.api.event.connection.DisconnectEvent
import com.velocitypowered.api.event.connection.PostLoginEvent
import com.velocitypowered.api.event.player.PlayerChatEvent
import com.velocitypowered.api.event.proxy.ProxyShutdownEvent
import org.slf4j.Logger
import java.io.File
import java.nio.file.Files


class Events(private val logger: Logger) {
    init {
        val libName = "libscharsch_bot_velocity"
        var libExtension = ".so"

        val osName = System.getProperty("os.name")

        if (osName.contains("Windows")) {
            libExtension = ".dll"
        } else if (osName.contains("Mac")) {
            libExtension = ".dylib"
        }
        val libDir = Files.createTempDirectory("ScharschBot")
        libDir.toFile().deleteOnExit()
        val libFile = File(libDir.toFile(), libName)


        javaClass.classLoader.getResourceAsStream(libName + libExtension).use { input ->
            if (input == null) {
                throw RuntimeException("Could not find ScharschBot library $libName")
            }
            Files.copy(input, libFile.toPath())
        }
        System.load(libFile.absolutePath)
        logger.info("Loaded ScharschBot library $libName")

        logger.info("Initializing ScharschBot core")

        Thread {
            onInitialize()
        }.start() // TODO: Do threading in Rust
    }

    private external fun onInitialize()

    @Subscribe
    external fun onPlayerJoin(event: PostLoginEvent)

    @Subscribe
    external fun onPlayerLeave(event: DisconnectEvent)

    @Subscribe
    external fun onPlayerChat(event: PlayerChatEvent)

    @Subscribe
    external fun onProxyShutdown(event: ProxyShutdownEvent)
}