# UniFFI's generated JNA callback and symbol names are part of the native ABI.
-keep class io.github.rikirsti.protunnext.generated.** { *; }
-keep class com.sun.jna.** { *; }
-dontwarn java.awt.**
