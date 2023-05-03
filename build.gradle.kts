plugins {
    java
    kotlin("jvm") version "1.7.20"
    id ("com.github.johnrengelman.shadow") version "8.1.1"
}
configurations {
    create("kotlin")
}

repositories {
    mavenLocal()
    maven {
        name = "papermc"
        url = uri("https://repo.papermc.io/repository/maven-public/")
    }

    maven("https://oss.sonatype.org/content/groups/public/")


    dependencies {
        compileOnly("com.velocitypowered:velocity-api:3.2.0-SNAPSHOT")
        annotationProcessor("com.velocitypowered:velocity-annotation-processor:3.2.0-SNAPSHOT")
    }

    group = "de.scharschbot"
    version = "0.1.0-SNAPSHOT"
    description = "Scharschbot Plugin for Velocity"
    java.sourceCompatibility = JavaVersion.VERSION_1_8
    sourceSets {
        main {
            kotlin {
                srcDirs("src/main/kotlin")
            }
            resources {
                srcDirs("src/main/jniLibs")
            }
        }
    }

//    tasks.named<ShadowJar>("shadowJar") {
//        configurations = listOf(project.configurations.getByName("kotlin"), project.configurations.getByName("compileClasspath"))
//    }
}
tasks {
    compileJava {
        options.encoding = "UTF-8"
    }
    compileKotlin {
        kotlinOptions.jvmTarget = "1.8"
    }
    build {
        dependsOn("shadowJar")
    }

    shadowJar {
        manifest {
            attributes["Main-Class"] = "de.scharschbot.velocity.plugin.Plugin"
        }
        relocate("kotlin", "de.scharschbot.velocity.plugin.kotlin")
    }
}


kotlin {
    jvmToolchain(17)
}

tasks.register("generateTemplates") {
}