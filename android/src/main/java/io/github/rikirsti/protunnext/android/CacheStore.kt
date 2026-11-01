package io.github.rikirsti.protunnext.android

import android.content.Context
import android.util.Base64
import io.github.rikirsti.protunnext.generated.CacheKey
import io.github.rikirsti.protunnext.generated.PersistentCache
import java.security.MessageDigest

interface PersistentCacheCipher {
    fun encrypt(bytes: ByteArray): Result<ByteArray>
    fun decrypt(bytes: ByteArray): Result<ByteArray>
}

/** Account callbacks run off the packet path. Never persist plaintext after a cipher failure. */
internal class CacheStore(context: Context, private val cipher: PersistentCacheCipher?) {
    private val prefs = context.getSharedPreferences("protun-next-cache", Context.MODE_PRIVATE)
    private val memory = HashMap<String, ByteArray>()
    private val lock = Any()
    private fun prefix(userId: String): String = MessageDigest.getInstance("SHA-256")
        .digest(userId.toByteArray(Charsets.UTF_8)).joinToString("") { "%02x".format(it) } + ":"
    fun forUser(userId: String): PersistentCache {
        val prefix = prefix(userId)
        return object : PersistentCache {
            override fun put(key: CacheKey, bytes: ByteArray) = synchronized(lock) {
                require(bytes.size <= 1024 * 1024) { "cache value too large" }
                val name = prefix + key.name
                memory.put(name, bytes.copyOf())?.fill(0)
                val stored = if (cipher == null) bytes.copyOf()
                    else runCatching { cipher.encrypt(bytes.copyOf()).getOrThrow() }.getOrNull()
                try {
                    val edit = prefs.edit().remove(name)
                    if (stored != null) edit.putString(name, Base64.encodeToString(stored, Base64.NO_WRAP))
                    edit.commit()
                } finally { stored?.fill(0) }
                Unit
            }
            override fun get(key: CacheKey): ByteArray? = synchronized(lock) {
                val name = prefix + key.name
                memory[name]?.let { return@synchronized it.copyOf() }
                val encoded = prefs.getString(name, null) ?: return@synchronized null
                if (encoded.length > 2 * 1024 * 1024) return@synchronized null
                val stored = runCatching { Base64.decode(encoded, Base64.NO_WRAP) }.getOrNull()
                    ?: return@synchronized null
                val value = try {
                    if (cipher == null) stored.copyOf()
                    else runCatching { cipher.decrypt(stored).getOrThrow().copyOf() }.getOrNull()
                } finally { stored.fill(0) }
                if (value == null || value.size > 1024 * 1024) {
                    value?.fill(0)
                    return@synchronized null
                }
                memory[name] = value
                value.copyOf()
            }
            override fun remove(key: CacheKey) = synchronized(lock) {
                val name = prefix + key.name
                memory.remove(name)?.fill(0)
                prefs.edit().remove(name).commit()
                Unit
            }
            override fun clearAll() = synchronized(lock) {
                val edit = prefs.edit()
                CacheKey.entries.forEach { key ->
                    val name = prefix + key.name
                    memory.remove(name)?.fill(0)
                    edit.remove(name)
                }
                edit.commit()
                Unit
            }
        }
    }
}
