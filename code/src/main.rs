
/* code für IP_Kamera */



/* 
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!(" Starting Jetson Hardware Streaming Node (UDP Mode)...");
    println!(" Engaging NVDEC & NVENC Hardware Accelerators...");


 





 let mut gst_process = Command::new("gst-launch-1.0")
 .args([
     "rtspsrc", "location=rtsp://Mechlab:mechlab@192.168.1.10/axis-media/media.amp", "latency=0", "drop-on-latency=true",
     "!", "rtph264depay", 
     "!", "h264parse",   
     "!", "rtph264pay", "config-interval=1", "pt=96", 
     "!", "udpsink", "host=100.104.68.38", "port=5000", "sync=false" 
 ])
 .stdout(Stdio::null())
 .stderr(Stdio::null())
 .spawn()
 .expect("Failed to start Jetson GStreamer process");
 
    println!("JETSON IS STREAMING DATA TO LAPTOP!");
    println!("Press [Ctrl + C] to safely stop the engine.");
   

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        sleep(Duration::from_millis(200));
    }

    println!("\n Shutting down transmission...");
    let _ = gst_process.kill();
    let _ = gst_process.wait();
    println!("Transmission stopped. Tschüss!");
}




*/


/*  code for USB-Camera */

/* 
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("🚀 Starting Autonomous Car Streaming Engine...");

    // 1. Release the camera port to prevent 'device busy' errors
    println!("🧹 Cleaning up old camera processes...");
    let _ = Command::new("sudo")
        .args(["fuser", "-k", "/dev/video0"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    sleep(Duration::from_secs(1));

    // 2. Start MediaMTX (WebRTC Translator/Server) via Docker in the background
    println!("🐳 Starting MediaMTX WebRTC Server (The Translator)...");
    let mut docker_process = Command::new("sudo")
        .args([
            "docker", "run", "--rm", "--network=host", "bluenviron/mediamtx:latest"
        ])
        .stdout(Stdio::null()) // Suppress Docker logs for a cleaner terminal
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start Docker process");

    // Wait for the Docker container to fully initialize
    sleep(Duration::from_secs(3));

    // 3. Start GStreamer using NVIDIA Hardware Encoder (Zero-Latency RTSP)
    // Optimized for crowded Wi-Fi (agdsn) with max performance and 800k bitrate
    println!("🎥 Starting NVIDIA Hardware Encoder & Camera...");
    let mut gst_process = Command::new("gst-launch-1.0")
        .args([
            "v4l2src", "device=/dev/video0",
            "!", "videoconvert",
            "!", "video/x-raw,width=640,height=480",
            "!", "nvvideoconvert",
            "!", "nvv4l2h264enc", "bitrate=800000", "insert-sps-pps=true", "idrinterval=15", "iframeinterval=15", "maxperf-enable=true", "preset-level=1",
            "!", "h264parse",
            "!", "rtspclientsink", "location=rtsp://localhost:8554/mystream"
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start GStreamer process");

    // Print success message and instructions
    println!("\n==================================================");
    println!("✅ SYSTEM IS LIVE AND STREAMING! (LOW LATENCY MODE)");
    println!("🌐 Open Browser on Laptop at: http://<JETSON_IP>:8889/mystream");
    println!("🔴 Press [Ctrl + C] to safely stop the engine.");
    println!("==================================================\n");

    // 4. Setup Graceful Shutdown Handler (Ctrl + C)
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    // Keep the main thread alive while streaming
    while running.load(Ordering::SeqCst) {
        sleep(Duration::from_millis(200));
    }

    // 5. Cleanup phase: Kill processes and free hardware resources safely
    println!("\n🛑 Shutting down safely...");
    let _ = gst_process.kill();
    let _ = docker_process.kill();
    
    // Wait for processes to terminate completely
    let _ = gst_process.wait();
    let _ = docker_process.wait();

    // Final camera cleanup
    let _ = Command::new("sudo")
        .args(["fuser", "-k", "/dev/video0"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    println!("✅ Shutdown complete. Goodbye!");
}


*/
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!(" Starting Autonomous Car Streaming Engine...");

    // 1. Release the camera port to prevent 'device busy' errors
    println!(" Cleaning up old camera processes...");
    let _ = Command::new("sudo")
        .args(["fuser", "-k", "/dev/video0"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    sleep(Duration::from_secs(1));

    // 2. Start MediaMTX (WebRTC Translator/Server) via Docker in the background
    println!(" Starting MediaMTX WebRTC Server (The Translator)...");
    let mut docker_process = Command::new("sudo")
        .args([
            "docker", "run", "--rm", "--network=host", "bluenviron/mediamtx:latest"
        ])
        .stdout(Stdio::null()) // Suppress Docker logs for a cleaner terminal
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start Docker process");

    // Wait for the Docker container to fully initialize
    sleep(Duration::from_secs(3));

    println!(" Starting NVIDIA Hardware Encoder & Camera...");
    let mut gst_process = Command::new("gst-launch-1.0")
        .args([
            "v4l2src", "device=/dev/video0",
            "!", "video/x-raw,width=640,height=480",
            "!", "nvvideoconvert",
            "!", "video/x-raw(memory:NVMM), format=NV12, width=640, height=480",
            "!", "nvv4l2h264enc", "bitrate=10000000", "insert-sps-pps=true", "idrinterval=15", "iframeinterval=15", "maxperf-enable=true", "preset-level=1",
            "!", "h264parse",
            "!", "rtspclientsink", "location=rtsp://localhost:8554/mystream"
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start GStreamer process");

    // Print success message and instructions
    println!("\n==================================================");
    println!(" SYSTEM IS LIVE AND STREAMING! (LOW LATENCY MODE)");
    println!(" Open Browser on Laptop at: http://<JETSON_IP>:8889/mystream");
    println!(" Press [Ctrl + C] to safely stop the engine.");
    println!("==================================================\n");

    // 4. Setup Graceful Shutdown Handler (Ctrl + C)
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    // Keep the main thread alive while streaming
    while running.load(Ordering::SeqCst) {
        sleep(Duration::from_millis(200));
    }

    // 5. Cleanup phase: Kill processes and free hardware resources safely
    println!("\n Shutting down safely...");
    let _ = gst_process.kill();
    let _ = docker_process.kill();
    
    // Wait for processes to terminate completely
    let _ = gst_process.wait();
    let _ = docker_process.wait();

    // Final camera cleanup
    let _ = Command::new("sudo")
        .args(["fuser", "-k", "/dev/video0"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    println!("✅ Shutdown complete. Tschüss!");
}


