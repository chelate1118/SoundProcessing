package sound.rust

import java.io.*
import java.net.*

object Rust {
    const val PORT: Int = 13924
    const val PATH: String = "..\\RustCore"
    
    lateinit var socket: Socket
    lateinit var outputStream: OutputStream
    lateinit var bufferedInputStream: BufferedInputStream
    
    fun start() {
//        val buildProcess = ProcessBuilder("cargo", "build", "--manifest-path", "$PATH\\Cargo.toml", "-r", "--", "$PORT")
//        buildProcess.start().waitFor()

//        val runProcess = ProcessBuilder("$PATH\\target\\release\\rust_core.exe", "$PORT")
//        runProcess.start()

        println("Accepting")

        val serverSocket = ServerSocket(PORT)
            socket = serverSocket.accept()
            println("connected")

        outputStream = socket.getOutputStream()

        val printWriter = PrintWriter(socket.getOutputStream())

        with (printWriter) {
                println("From Kotlin: 9099")
                flush()
        }
    }
}