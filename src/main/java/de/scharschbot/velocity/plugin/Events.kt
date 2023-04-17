package de.scharschbot.velocity.plugin

import com.velocitypowered.api.event.Subscribe
import com.velocitypowered.api.event.connection.DisconnectEvent
import com.velocitypowered.api.event.connection.PostLoginEvent
import com.velocitypowered.api.event.proxy.ProxyInitializeEvent
import org.slf4j.Logger
import java.io.File
import java.nio.file.Files
import java.nio.file.Paths


class Events(logger: Logger) {
    init {
        val libName = "libscharsch_bot_velocity"
        var libExtension = ".so"

        val osName = System.getProperty("os.name")

        if (osName.contains("Windows")) {
            libExtension = ".dll";
        } else if (osName.contains("Mac")) {
            libExtension = ".dylib";
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
        val libPath = Paths.get("./plugins/scharschbot/libscharsch_bot_velocity.so")
        System.load(libPath.toAbsolutePath().toString())
        logger.info("Loaded ScharschBot library $libName")


        onInitialize()
    }

    private external fun onInitialize()

    @Subscribe
    external fun onPlayerJoin(event: PostLoginEvent)

    @Subscribe
    external fun onPlayerLeave(event: DisconnectEvent)
}