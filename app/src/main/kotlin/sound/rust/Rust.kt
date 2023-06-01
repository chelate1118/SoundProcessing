package sound.rust

import java.net.*

object Rust {
    const val PORT: Int = 4944
    const val PATH: String = "D:\\SoundProcess\\RustCore\\Cargo.toml"
    
    lateinit var serverSocket: ServerSocket
    lateinit var socket: Socket
    
    fun start() {
        serverSocket = ServerSocket(PORT)
        ProcessBuilder("cargo", "run", "--manifest-path", PATH, "-r", "--", PORT.toString()).start()
        socket = serverSocket.accept()
    }
}