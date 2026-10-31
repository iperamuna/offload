<template>
  <!-- Outer Window Container (Transparent with padding for drop shadow) -->
  <div 
    class="h-full w-full bg-transparent flex flex-col items-center justify-start p-2 select-none font-sans overflow-hidden transition-colors"
    :class="{ 'dark': isDarkMode }"
  >
    
    <!-- Top Caret Notch (Crisp macOS Menu Bar Arrow) -->
    <div class="flex justify-center w-full shrink-0 -mb-[1px] z-30">
      <svg width="20" height="8" viewBox="0 0 20 8" fill="none">
        <path 
          d="M0 8 L8.5 1.2 C9.4 0.5 10.6 0.5 11.5 1.2 L20 8 Z" 
          :fill="isDarkMode ? '#18181b' : '#ffffff'" 
          :stroke="isDarkMode ? '#27272a' : '#e4e4e7'" 
          stroke-width="1" 
        />
        <rect 
          x="0" 
          y="7" 
          width="20" 
          height="2" 
          :fill="isDarkMode ? '#18181b' : '#ffffff'" 
        />
      </svg>
    </div>

    <!-- Main Card Container: Standard macOS Corner Radius (14px) Solid Opaque Card -->
    <div 
      class="w-full flex-1 rounded-xl flex flex-col overflow-hidden transition-colors duration-150 z-20"
      :class="isDarkMode 
        ? 'bg-[#18181b] border border-zinc-800 shadow-[0_24px_50px_rgba(0,0,0,0.8)] text-zinc-100' 
        : 'bg-white border border-zinc-200/90 shadow-[0_20px_45px_rgba(0,0,0,0.18),0_2px_8px_rgba(0,0,0,0.06)] text-zinc-800'"
    >
      
      <!-- Top Titlebar Header -->
      <div 
        class="flex items-center justify-between px-3.5 py-2.5 border-b shrink-0 transition-colors"
        :class="isDarkMode ? 'border-zinc-800 bg-zinc-900' : 'border-zinc-200 bg-zinc-50'"
      >
        <!-- App Identity -->
        <div class="flex items-center gap-2">
          <div 
            class="relative w-5.5 h-5.5 rounded-md flex items-center justify-center p-1 shadow-sm transition-colors"
            :class="isDarkMode ? 'bg-zinc-800 text-zinc-100 border border-white/10' : 'bg-zinc-900 text-white'"
          >
            <HardDrive class="w-3.5 h-3.5" />
            <div 
              class="absolute -top-0.5 -right-0.5 w-1.5 h-1.5 rounded-full border"
              :class="[
                isDriveConnected ? 'bg-emerald-500' : 'bg-amber-500',
                isDarkMode ? 'border-[#141416]' : 'border-white'
              ]"
            ></div>
          </div>
          <div class="flex items-center gap-1.5">
            <span class="text-xs font-bold tracking-tight" :class="isDarkMode ? 'text-white' : 'text-zinc-900'">Offload</span>
            <span 
              class="text-[9px] font-semibold uppercase px-1 py-0.2 rounded font-mono border"
              :class="isDarkMode 
                ? 'bg-zinc-800/80 text-zinc-300 border-white/10' 
                : 'bg-zinc-100 text-zinc-600 border-zinc-200'"
            >
              v1.0
            </span>
          </div>
        </div>

        <!-- Quick Header Action Buttons -->
        <div class="flex items-center gap-1.5">
          <!-- Permissions indicator badge: Only shown if action is required -->
          <button 
            v-if="!permissions.all_granted"
            @click="activeView = 'permissions'" 
            title="macOS System Permissions Action Required"
            class="p-1 rounded-md transition-colors border text-xs"
            :class="isDarkMode 
              ? 'bg-amber-500/15 text-amber-300 border-amber-500/30 animate-pulse' 
              : 'bg-amber-50 text-amber-700 border-amber-300 animate-pulse'"
          >
            <ShieldAlert class="w-3.5 h-3.5" />
          </button>

          <!-- Theme Toggle (System / Light / Dark) -->
          <button 
            @click="cycleTheme"
            :title="`Appearance: ${themeChoice} (Click to change)`"
            class="p-1 rounded-md transition-colors border"
            :class="isDarkMode 
              ? 'bg-zinc-800/80 hover:bg-zinc-700 text-zinc-300 border-white/[0.08]' 
              : 'bg-zinc-100 hover:bg-zinc-200 text-zinc-700 border-zinc-200'"
          >
            <Sun v-if="themeChoice === 'light'" class="w-3.5 h-3.5 text-amber-500" />
            <Moon v-else-if="themeChoice === 'dark'" class="w-3.5 h-3.5 text-blue-400" />
            <Laptop v-else class="w-3.5 h-3.5 text-zinc-400" />
          </button>

          <!-- Settings Cog Toggle -->
          <button 
            @click="activeView = activeView === 'settings' ? 'offload' : 'settings'" 
            title="Destination & App Settings"
            class="p-1 rounded-md transition-colors border"
            :class="activeView === 'settings' 
              ? (isDarkMode ? 'bg-zinc-700 text-white border-white/20' : 'bg-zinc-900 text-white border-zinc-900') 
              : (isDarkMode ? 'bg-zinc-800/80 hover:bg-zinc-700 text-zinc-300 border-white/[0.08]' : 'bg-zinc-100 hover:bg-zinc-200 text-zinc-700 border-zinc-200')"
          >
            <Settings class="w-3.5 h-3.5" />
          </button>

          <!-- Refresh -->
          <button 
            @click="refreshDiskStats" 
            title="Refresh statistics"
            class="p-1 rounded-md transition-colors border"
            :class="isDarkMode 
              ? 'bg-zinc-800/80 hover:bg-zinc-700 text-zinc-300 border-white/[0.08]' 
              : 'bg-zinc-100 hover:bg-zinc-200 text-zinc-700 border-zinc-200'"
          >
            <RefreshCw class="w-3.5 h-3.5" :class="{ 'animate-spin': isRefreshing }" />
          </button>

          <!-- Quit -->
          <button 
            @click="quitApplication" 
            title="Quit Offload"
            class="p-1 rounded-md transition-colors border"
            :class="isDarkMode 
              ? 'bg-zinc-800/80 hover:bg-rose-500/20 hover:text-rose-400 text-zinc-400 border-white/[0.08]' 
              : 'bg-zinc-100 hover:bg-rose-50 hover:text-rose-600 text-zinc-500 border-zinc-200'"
          >
            <Power class="w-3.5 h-3.5" />
          </button>
        </div>
      </div>

      <!-- ============================================================= -->
      <!-- VIEW: PERMISSION SETUP SCREEN (Only if permissions are missing) -->
      <!-- ============================================================= -->
      <div v-if="activeView === 'permissions' && !permissions.all_granted" class="flex-1 flex flex-col p-3.5 justify-between overflow-y-auto">
        <div class="space-y-3">
          <div class="text-center py-1">
            <div 
              class="w-10 h-10 rounded-xl flex items-center justify-center mx-auto mb-2 border shadow-sm transition-colors"
              :class="isDarkMode 
                ? 'bg-zinc-800/80 text-zinc-200 border-white/10' 
                : 'bg-zinc-100 text-zinc-800 border-zinc-200'"
            >
              <ShieldCheck class="w-5 h-5" />
            </div>
            <h2 class="text-sm font-bold" :class="isDarkMode ? 'text-white' : 'text-zinc-900'">macOS Permissions Setup</h2>
            <p class="text-[11px] max-w-xs mx-auto mt-0.5 leading-relaxed" :class="isDarkMode ? 'text-zinc-400' : 'text-zinc-500'">
              Apple requires explicit authorization so Offload can relocate protected directories and maintain symlinks without errors.
            </p>
          </div>

          <!-- Permission Item 1: Full Disk Access -->
          <div 
            class="border rounded-xl p-3 space-y-2 transition-colors" 
            :class="[
              permissions.full_disk_access 
                ? (isDarkMode ? 'bg-zinc-900/60 border-emerald-500/30' : 'bg-emerald-50/40 border-emerald-300') 
                : (isDarkMode ? 'bg-zinc-900/60 border-amber-500/40' : 'bg-amber-50/50 border-amber-300')
            ]"
          >
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <Laptop class="w-4 h-4" :class="permissions.full_disk_access ? 'text-emerald-500' : 'text-amber-500'" />
                <span class="text-xs font-bold" :class="isDarkMode ? 'text-zinc-100' : 'text-zinc-900'">Full Disk Access</span>
              </div>
              <span 
                class="text-[9px] font-bold px-2 py-0.5 rounded-full uppercase tracking-wider border font-mono"
                :class="permissions.full_disk_access 
                  ? (isDarkMode ? 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20' : 'bg-emerald-100 text-emerald-800 border-emerald-200') 
                  : (isDarkMode ? 'bg-amber-500/10 text-amber-300 border-amber-500/20' : 'bg-amber-100 text-amber-800 border-amber-200')"
              >
                {{ permissions.full_disk_access ? 'Granted' : 'Required' }}
              </span>
            </div>
            <p class="text-[10px] leading-normal" :class="isDarkMode ? 'text-zinc-400' : 'text-zinc-600'">
              Allows moving privileged directories like iOS Backups, Docker containers, and developer caches without <span class="text-rose-500 font-mono">Operation not permitted</span> errors.
            </p>
            <button 
              @click="openSettings('fda')"
              class="w-full py-1.5 px-2.5 rounded-lg text-[11px] font-semibold border flex items-center justify-center gap-1.5 transition-colors"
              :class="isDarkMode 
                ? 'bg-zinc-800 hover:bg-zinc-700 text-zinc-200 border-white/10' 
                : 'bg-white hover:bg-zinc-50 text-zinc-800 border-zinc-200 shadow-sm'"
            >
              <span>Open System Settings → Full Disk Access</span>
              <ExternalLink class="w-3 h-3 opacity-60" />
            </button>
          </div>

          <!-- Permission Item 2: External Drive Access -->
          <div 
            class="border rounded-xl p-3 space-y-2 transition-colors" 
            :class="[
              permissions.external_drive_access 
                ? (isDarkMode ? 'bg-zinc-900/60 border-emerald-500/30' : 'bg-emerald-50/40 border-emerald-300') 
                : (isDarkMode ? 'bg-zinc-900/60 border-white/[0.08]' : 'bg-zinc-50 border-zinc-200')
            ]"
          >
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <HardDrive class="w-4 h-4" :class="permissions.external_drive_access ? 'text-emerald-500' : 'text-zinc-400'" />
                <span class="text-xs font-bold" :class="isDarkMode ? 'text-zinc-100' : 'text-zinc-900'">External Drive Access</span>
              </div>
              <span 
                class="text-[9px] font-bold px-2 py-0.5 rounded-full uppercase tracking-wider border font-mono"
                :class="permissions.external_drive_access 
                  ? (isDarkMode ? 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20' : 'bg-emerald-100 text-emerald-800 border-emerald-200') 
                  : (isDarkMode ? 'bg-zinc-800 text-zinc-400 border-white/10' : 'bg-zinc-200 text-zinc-600 border-zinc-300')"
              >
                {{ permissions.external_drive_access ? 'Verified' : 'Check Drive' }}
              </span>
            </div>
            <p class="text-[10px] leading-normal" :class="isDarkMode ? 'text-zinc-400' : 'text-zinc-600'">
              Allows writing sparse APFS files and managing mirrors on <span class="font-mono font-semibold" :class="isDarkMode ? 'text-zinc-200' : 'text-zinc-800'">{{ currentVolumeName }}</span>.
            </p>
          </div>
        </div>

        <!-- Footer Actions -->
        <div 
          class="pt-3 border-t flex items-center gap-2"
          :class="isDarkMode ? 'border-white/[0.08]' : 'border-zinc-200'"
        >
          <button 
            @click="verifyPermissionsLive"
            class="flex-1 py-2 rounded-lg text-xs font-semibold border flex items-center justify-center gap-1.5 transition-colors"
            :class="isDarkMode 
              ? 'bg-zinc-800 hover:bg-zinc-700 text-zinc-200 border-white/10' 
              : 'bg-zinc-100 hover:bg-zinc-200 text-zinc-800 border-zinc-200'"
          >
            <RefreshCw class="w-3 h-3" :class="{ 'animate-spin': isCheckingPermissions }" />
            <span>Re-Check Status</span>
          </button>
          <button 
            @click="dismissPermissionsScreen"
            class="flex-1 py-2 rounded-lg text-xs font-bold shadow-sm transition-all"
            :class="isDarkMode 
              ? 'bg-white hover:bg-zinc-200 text-zinc-950' 
              : 'bg-zinc-900 hover:bg-zinc-800 text-white'"
          >
            <span>Continue to App</span>
          </button>
        </div>
      </div>

      <!-- ============================================================= -->
      <!-- NORMAL APP WORKFLOWS -->
      <!-- ============================================================= -->
      <template v-else>
        <!-- Storage Metrics Cards (/System/Volumes/Data accurate values) -->
        <div 
          class="p-2.5 border-b grid grid-cols-2 gap-2 shrink-0 transition-colors"
          :class="isDarkMode ? 'bg-zinc-950/30 border-white/[0.06]' : 'bg-zinc-50/60 border-zinc-200/80'"
        >
          <!-- Internal Mac SSD -->
          <div 
            class="border rounded-lg p-2 flex flex-col justify-between transition-colors"
            :class="isDarkMode ? 'bg-zinc-900/50 border-white/[0.06]' : 'bg-white border-zinc-200/80 shadow-xs'"
          >
            <div class="flex items-center justify-between mb-1">
              <span class="text-[11px] font-bold flex items-center gap-1.5" :class="isDarkMode ? 'text-zinc-200' : 'text-zinc-800'">
                <Laptop class="w-3 h-3 opacity-70" />
                Mac SSD
              </span>
              <span class="text-[10px] font-bold font-mono" :class="internalUsedPercent > 90 ? 'text-rose-500' : (isDarkMode ? 'text-zinc-300' : 'text-zinc-700')">
                {{ diskStats.internal.free }} Free
              </span>
            </div>
            <!-- Bar -->
            <div 
              class="w-full h-1.5 rounded-full overflow-hidden my-1"
              :class="isDarkMode ? 'bg-zinc-800' : 'bg-zinc-200'"
            >
              <div 
                class="h-full rounded-full transition-all duration-300"
                :class="internalUsedPercent > 90 ? 'bg-rose-500' : (isDarkMode ? 'bg-zinc-200' : 'bg-zinc-800')"
                :style="{ width: internalUsedPercent + '%' }"
              ></div>
            </div>
            <div class="flex justify-between items-center text-[9px] font-mono" :class="isDarkMode ? 'text-zinc-400' : 'text-zinc-500'">
              <span>{{ diskStats.internal.used }} used</span>
              <span :class="internalUsedPercent > 90 ? 'text-rose-500 font-bold' : ''">{{ internalUsedPercent }}% full</span>
            </div>
          </div>

          <!-- External Target Volume -->
          <div 
            class="border rounded-lg p-2 flex flex-col justify-between transition-colors"
            :class="isDarkMode ? 'bg-zinc-900/50 border-white/[0.06]' : 'bg-white border-zinc-200/80 shadow-xs'"
          >
            <div class="flex items-center justify-between mb-1">
              <span class="text-[11px] font-bold flex items-center gap-1.5 truncate max-w-[95px]" :title="currentVolumeName" :class="isDarkMode ? 'text-zinc-200' : 'text-zinc-800'">
                <HardDrive class="w-3 h-3 text-emerald-500 shrink-0" />
                <span class="truncate">{{ currentVolumeName }}</span>
              </span>
              <span class="text-[10px] font-bold text-emerald-500 font-mono">
                {{ diskStats.external.free }} Free
              </span>
            </div>
            <!-- Bar -->
            <div 
              class="w-full h-1.5 rounded-full overflow-hidden my-1"
              :class="isDarkMode ? 'bg-zinc-800' : 'bg-zinc-200'"
            >
              <div 
                class="h-full bg-emerald-500 rounded-full transition-all duration-300"
                :style="{ width: externalUsedPercent + '%' }"
              ></div>
            </div>
            <div class="flex justify-between items-center text-[9px] font-mono" :class="isDarkMode ? 'text-zinc-400' : 'text-zinc-500'">
              <span>{{ diskStats.external.used }} used</span>
              <span>{{ externalUsedPercent }}% full</span>
            </div>
          </div>
        </div>

        <!-- Segmented Tab Navigation: Offload | Symlinks | Settings -->
        <div class="px-2.5 pt-2 pb-1 shrink-0">
          <div 
            class="grid grid-cols-3 p-0.5 rounded-lg border transition-colors"
            :class="isDarkMode ? 'bg-zinc-950/60 border-white/[0.06]' : 'bg-zinc-200/60 border-zinc-200'"
          >
            <!-- Tab 1 -->
            <button 
              @click="activeView = 'offload'"
              :class="activeView === 'offload' 
                ? (isDarkMode ? 'bg-zinc-800 text-white font-semibold shadow-xs border border-white/10' : 'bg-white text-zinc-900 font-semibold shadow-xs') 
                : (isDarkMode ? 'text-zinc-400 hover:text-zinc-200' : 'text-zinc-600 hover:text-zinc-900')"
              class="py-1 rounded-md text-[11px] transition-all flex items-center justify-center gap-1"
            >
              <FolderDown class="w-3 h-3" />
              <span>Offload</span>
            </button>
            <!-- Tab 2 -->
            <button 
              @click="activeView = 'active'"
              :class="activeView === 'active' 
                ? (isDarkMode ? 'bg-zinc-800 text-white font-semibold shadow-xs border border-white/10' : 'bg-white text-zinc-900 font-semibold shadow-xs') 
                : (isDarkMode ? 'text-zinc-400 hover:text-zinc-200' : 'text-zinc-600 hover:text-zinc-900')"
              class="py-1 rounded-md text-[11px] transition-all flex items-center justify-center gap-1"
            >
              <ArrowRightLeft class="w-3 h-3" />
              <span>Symlinks</span>
              <span 
                v-if="activeSymlinks.length" 
                class="text-[9px] px-1 py-0.1 rounded-full font-bold font-mono"
                :class="isDarkMode ? 'bg-zinc-700 text-zinc-200' : 'bg-zinc-200 text-zinc-700'"
              >
                {{ activeSymlinks.length }}
              </span>
            </button>
            <!-- Tab 3 -->
            <button 
              @click="activeView = 'settings'"
              :class="activeView === 'settings' 
                ? (isDarkMode ? 'bg-zinc-800 text-white font-semibold shadow-xs border border-white/10' : 'bg-white text-zinc-900 font-semibold shadow-xs') 
                : (isDarkMode ? 'text-zinc-400 hover:text-zinc-200' : 'text-zinc-600 hover:text-zinc-900')"
              class="py-1 rounded-md text-[11px] transition-all flex items-center justify-center gap-1"
            >
              <Settings class="w-3 h-3" />
              <span>Settings</span>
            </button>
          </div>
        </div>

        <!-- Fluid Main Content Viewport -->
        <div class="flex-1 overflow-y-auto px-2.5 py-1.5 min-h-0 space-y-2">
          
          <!-- ============================================================= -->
          <!-- VIEW 1: OFFLOAD WORKFLOW -->
          <!-- ============================================================= -->
          <div v-if="activeView === 'offload'" class="flex flex-col gap-2">
            
            <!-- Target Summary Bar -->
            <div 
              class="px-2.5 py-1.5 rounded-lg border flex items-center justify-between text-[11px] transition-colors"
              :class="isDarkMode ? 'bg-zinc-900/40 border-white/[0.06]' : 'bg-zinc-50 border-zinc-200/80'"
            >
              <div class="flex items-center gap-1.5 truncate">
                <span class="text-[10px] uppercase font-bold tracking-wider opacity-60">Target:</span>
                <span class="font-mono font-semibold text-[10px] truncate max-w-[190px]" :title="`${targetVolumePath}/${mirrorSubfolder}`">
                  {{ currentVolumeName }} / {{ mirrorSubfolder }}
                </span>
              </div>
              <button 
                @click="activeView = 'settings'" 
                class="text-[10px] font-medium hover:underline shrink-0"
                :class="isDarkMode ? 'text-zinc-400 hover:text-white' : 'text-zinc-600 hover:text-zinc-900'"
              >
                Edit Target
              </button>
            </div>

            <!-- Dropzone -->
            <div 
              @dragover.prevent="isDragging = true"
              @dragleave.prevent="isDragging = false"
              @drop.prevent="handleFolderDrop"
              :class="[
                isDragging 
                  ? (isDarkMode ? 'border-zinc-400 bg-zinc-800/40' : 'border-zinc-500 bg-zinc-100') 
                  : (isDarkMode ? 'border-white/[0.08] hover:border-zinc-600 bg-zinc-900/30' : 'border-zinc-300 hover:border-zinc-400 bg-zinc-50/50'),
                selectedFolder ? 'py-2 px-2.5' : 'py-4 px-3'
              ]"
              class="border-2 border-dashed rounded-lg flex flex-col items-center justify-center transition-all cursor-pointer text-center group"
              @click="selectFolderDialog"
            >
              <div v-if="!selectedFolder" class="flex flex-col items-center gap-1">
                <div 
                  class="w-7 h-7 rounded-lg flex items-center justify-center group-hover:scale-105 transition-transform"
                  :class="isDarkMode ? 'bg-zinc-800 text-zinc-300' : 'bg-zinc-100 text-zinc-700'"
                >
                  <FolderUp class="w-4 h-4" />
                </div>
                <div class="text-xs font-semibold" :class="isDarkMode ? 'text-zinc-200' : 'text-zinc-800'">
                  Drop folder here or click to browse
                </div>
                <p class="text-[10px]" :class="isDarkMode ? 'text-zinc-400' : 'text-zinc-500'">
                  Relocates to <span class="font-mono">{{ currentVolumeName }}/{{ mirrorSubfolder }}</span> with instant symlink
                </p>
              </div>

              <div v-else class="flex items-center justify-between w-full gap-2">
                <div class="flex items-center gap-2 text-left min-w-0">
                  <Folder class="w-4 h-4 shrink-0" :class="isDarkMode ? 'text-zinc-300' : 'text-zinc-700'" />
                  <div class="min-w-0">
                    <div class="text-xs font-bold flex items-center gap-1.5" :class="isDarkMode ? 'text-white' : 'text-zinc-900'">
                      <span class="truncate">{{ folderName }}</span>
                      <span 
                        class="text-[10px] px-1.5 py-0.2 rounded font-mono shrink-0"
                        :class="isDarkMode ? 'bg-zinc-800 text-zinc-300' : 'bg-zinc-100 text-zinc-700'"
                      >
                        {{ folderSize }}
                      </span>
                    </div>
                    <div class="text-[9px] font-mono truncate max-w-[220px]" :class="isDarkMode ? 'text-zinc-400' : 'text-zinc-500'">
                      {{ selectedFolder }}
                    </div>
                  </div>
                </div>
                <button 
                  @click.stop="resetSelectedFolder"
                  class="text-[10px] px-2 py-0.5 rounded transition-colors shrink-0"
                  :class="isDarkMode 
                    ? 'text-zinc-400 hover:text-rose-400 bg-zinc-800 hover:bg-zinc-700' 
                    : 'text-zinc-500 hover:text-rose-600 bg-zinc-200/70 hover:bg-zinc-200'"
                >
                  Change
                </button>
              </div>
            </div>

            <!-- Optional Exception Chips Panel (Keep on Internal SSD) -->
            <div 
              v-if="selectedFolder && !isProcessing" 
              class="border rounded-lg p-2.5 space-y-2 transition-colors"
              :class="isDarkMode ? 'bg-zinc-900/40 border-white/[0.06]' : 'bg-white border-zinc-200/80 shadow-xs'"
            >
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-1.5 text-[10px] font-bold uppercase tracking-wider" :class="isDarkMode ? 'text-zinc-300' : 'text-zinc-700'">
                  <ShieldCheck class="w-3.5 h-3.5 text-emerald-500" />
                  <span>Optional SSD Stashing (<code class="font-mono">--except</code>)</span>
                </div>
                <span class="text-[9px] font-mono opacity-60">
                  {{ selectedExceptions.length ? `${selectedExceptions.length} kept on SSD` : 'Full folder offload' }}
                </span>
              </div>
              <p class="text-[10px] leading-tight" :class="isDarkMode ? 'text-zinc-400' : 'text-zinc-500'">
                Offloads 100% of this folder by default. Select any items below only if you wish to keep them on internal SSD.
              </p>

              <!-- Chips list -->
              <div class="flex flex-wrap gap-1.5 max-h-24 overflow-y-auto pr-0.5">
                <button 
                  v-for="item in detectedSubitems" 
                  :key="item.name"
                  @click="toggleException(item.name)"
                  :class="selectedExceptions.includes(item.name) 
                    ? (isDarkMode ? 'bg-zinc-200 text-zinc-900 font-semibold' : 'bg-zinc-900 text-white font-semibold') 
                    : (isDarkMode ? 'bg-zinc-800/80 border border-white/[0.08] text-zinc-400 hover:text-zinc-200' : 'bg-zinc-100 border border-zinc-200 text-zinc-600 hover:text-zinc-900')"
                  class="px-2 py-1 rounded-md text-[10px] font-mono flex items-center gap-1.5 transition-all"
                >
                  <Check v-if="selectedExceptions.includes(item.name)" class="w-2.5 h-2.5 stroke-[3] shrink-0" />
                  <span class="truncate max-w-[120px]">{{ item.name }}</span>
                  <span class="text-[9px] opacity-70">({{ item.size }})</span>
                </button>
              </div>

              <!-- Custom Exception Input -->
              <div class="flex items-center gap-1.5 pt-0.5">
                <input 
                  v-model="customExceptionInput" 
                  @keydown.enter.prevent="addCustomException"
                  type="text" 
                  placeholder="Add custom exception (.env, cache)..." 
                  class="flex-1 rounded-md px-2 py-1 text-[10px] font-mono min-w-0 border focus:outline-none transition-colors"
                  :class="isDarkMode 
                    ? 'bg-zinc-950 border-white/[0.08] text-zinc-100 placeholder-zinc-500 focus:border-zinc-500' 
                    : 'bg-zinc-50 border-zinc-200 text-zinc-900 placeholder-zinc-400 focus:border-zinc-400'"
                />
                <button 
                  @click="addCustomException"
                  class="px-2.5 py-1 rounded-md text-[10px] font-semibold border transition-colors shrink-0"
                  :class="isDarkMode 
                    ? 'bg-zinc-800 hover:bg-zinc-700 text-zinc-200 border-white/10' 
                    : 'bg-zinc-100 hover:bg-zinc-200 text-zinc-800 border-zinc-200'"
                >
                  Add
                </button>
              </div>

              <!-- Start Action Button -->
              <button 
                @click="startOffloadAction"
                class="w-full mt-1 py-1.5 text-xs font-bold rounded-lg shadow-sm flex items-center justify-center gap-1.5 transition-all active:scale-[0.98]"
                :class="isDarkMode 
                  ? 'bg-white hover:bg-zinc-200 text-zinc-950 shadow-white/5' 
                  : 'bg-zinc-900 hover:bg-zinc-800 text-white'"
              >
                <Zap class="w-3.5 h-3.5 fill-current" />
                <span v-if="!selectedExceptions.length">Offload Entire Folder to {{ currentVolumeName }}</span>
                <span v-else>Offload to {{ currentVolumeName }} ({{ selectedExceptions.length }} kept on SSD)</span>
              </button>
            </div>

            <!-- Progress Card -->
            <div 
              v-if="isProcessing" 
              class="border rounded-lg p-2.5 text-center space-y-2 transition-colors"
              :class="isDarkMode ? 'bg-zinc-900/60 border-white/10' : 'bg-zinc-100 border-zinc-300'"
            >
              <div class="flex items-center justify-center gap-2 text-xs font-bold" :class="isDarkMode ? 'text-white' : 'text-zinc-900'">
                <RefreshCw class="w-3.5 h-3.5 animate-spin" />
                <span>{{ processTitle }}</span>
              </div>
              <div class="w-full h-1.5 rounded-full overflow-hidden" :class="isDarkMode ? 'bg-zinc-800' : 'bg-zinc-200'">
                <div 
                  class="h-full rounded-full transition-all duration-300"
                  :class="isDarkMode ? 'bg-white' : 'bg-zinc-900'"
                  :style="{ width: processPercentage + '%' }"
                ></div>
              </div>
              <div class="text-[10px] font-mono truncate" :class="isDarkMode ? 'text-zinc-400' : 'text-zinc-500'">
                {{ processStatusMessage }}
              </div>
            </div>
          </div>

          <!-- ============================================================= -->
          <!-- VIEW 2: ACTIVE SYMLINKS LIST -->
          <!-- ============================================================= -->
          <div v-else-if="activeView === 'active'" class="space-y-1.5">
            <div 
              v-for="item in activeSymlinks" 
              :key="item.path"
              class="border rounded-lg p-2 flex items-center justify-between gap-2 transition-colors"
              :class="isDarkMode ? 'bg-zinc-900/40 border-white/[0.06] hover:border-zinc-700' : 'bg-white border-zinc-200/80 hover:border-zinc-300 shadow-xs'"
            >
              <div class="min-w-0 flex-1">
                <div class="flex items-center gap-1.5">
                  <span class="text-xs font-bold truncate" :class="isDarkMode ? 'text-zinc-100' : 'text-zinc-900'">{{ item.name }}</span>
                  <span 
                    class="text-[9px] px-1.5 py-0.2 rounded-full font-mono font-semibold shrink-0"
                    :class="isDarkMode ? 'bg-emerald-500/10 text-emerald-400' : 'bg-emerald-50 text-emerald-700 border border-emerald-200'"
                  >
                    {{ item.size }}
                  </span>
                </div>
                <div class="text-[9px] font-mono truncate mt-0.5" :class="isDarkMode ? 'text-zinc-400' : 'text-zinc-500'">
                  {{ item.path }}
                </div>
              </div>

              <div class="flex items-center gap-1 shrink-0">
                <button 
                  @click="openInFinder(item.path)"
                  title="Reveal in Finder"
                  class="p-1 rounded-md transition-colors border"
                  :class="isDarkMode 
                    ? 'bg-zinc-800 hover:bg-zinc-700 text-zinc-300 border-white/5' 
                    : 'bg-zinc-100 hover:bg-zinc-200 text-zinc-700 border-zinc-200'"
                >
                  <ExternalLink class="w-3 h-3" />
                </button>
                <button 
                  @click="reverseItemAction(item.path)"
                  title="Restore to internal SSD"
                  class="p-1 rounded-md transition-colors border"
                  :class="isDarkMode 
                    ? 'bg-rose-500/10 hover:bg-rose-500/25 text-rose-400 border-rose-500/20' 
                    : 'bg-rose-50 hover:bg-rose-100 text-rose-600 border-rose-200'"
                >
                  <Undo2 class="w-3 h-3" />
                </button>
              </div>
            </div>

            <div v-if="!activeSymlinks.length" class="text-center py-8 text-[11px]" :class="isDarkMode ? 'text-zinc-500' : 'text-zinc-400'">
              No active offloaded symlinks detected.
            </div>
          </div>

          <!-- ============================================================= -->
          <!-- VIEW 3: SETTINGS & TARGET DESTINATION MANAGEMENT -->
          <!-- ============================================================= -->
          <div v-else-if="activeView === 'settings'" class="space-y-2.5">
            
            <!-- Target Destination Section -->
            <div 
              class="border rounded-lg p-2.5 space-y-2 transition-colors"
              :class="isDarkMode ? 'bg-zinc-900/40 border-white/[0.06]' : 'bg-white border-zinc-200/80 shadow-xs'"
            >
              <div class="flex items-center justify-between">
                <span class="text-xs font-bold" :class="isDarkMode ? 'text-white' : 'text-zinc-900'">
                  Target Volume & Subfolder
                </span>
                <span 
                  class="text-[9px] font-mono px-1.5 py-0.5 rounded border"
                  :class="isDarkMode ? 'bg-zinc-800 text-zinc-300 border-white/10' : 'bg-zinc-100 text-zinc-700 border-zinc-200'"
                >
                  Configured
                </span>
              </div>

              <!-- Selected Volume Dropdown -->
              <div class="space-y-1">
                <label class="text-[10px] font-medium opacity-70">Target Disk Volume</label>
                <div class="flex items-center gap-1.5">
                  <div class="relative flex-1">
                    <select 
                      v-model="targetVolumePath" 
                      @change="onVolumeChange"
                      class="w-full text-[11px] font-medium rounded-md pl-2 pr-6 py-1 appearance-none focus:outline-none cursor-pointer truncate border transition-colors"
                      :class="isDarkMode 
                        ? 'bg-zinc-800 border-white/10 text-zinc-200 focus:border-zinc-500' 
                        : 'bg-zinc-50 border-zinc-300 text-zinc-900 focus:border-zinc-400'"
                    >
                      <option v-for="vol in availableVolumes" :key="vol.path" :value="vol.path">
                        {{ vol.name }} ({{ vol.free }} free of {{ vol.total }})
                      </option>
                    </select>
                    <ChevronDown class="w-3 h-3 absolute right-1.5 top-2 pointer-events-none opacity-50" />
                  </div>
                  <button 
                    @click="browseCustomVolume"
                    title="Browse disk volume"
                    class="p-1 rounded-md border transition-colors shrink-0"
                    :class="isDarkMode ? 'bg-zinc-800 hover:bg-zinc-700 text-zinc-200 border-white/10' : 'bg-zinc-100 hover:bg-zinc-200 text-zinc-800 border-zinc-200'"
                  >
                    <FolderSearch class="w-3.5 h-3.5" />
                  </button>
                </div>
              </div>

              <!-- Destination Subfolder Input -->
              <div class="space-y-1">
                <div class="flex items-center justify-between">
                  <label class="text-[10px] font-medium opacity-70">Folder Inside Selected Volume</label>
                  <span class="text-[9px] font-mono opacity-50">Subfolder Name</span>
                </div>
                <div class="flex items-center gap-1.5">
                  <input 
                    v-model="mirrorSubfolder" 
                    @input="saveSubfolderSetting"
                    type="text" 
                    placeholder="e.g. Indunilperamuna-Macos" 
                    class="flex-1 font-mono text-[11px] rounded-md px-2 py-1 border focus:outline-none transition-colors"
                    :class="isDarkMode 
                      ? 'bg-zinc-950 border-white/10 text-zinc-100 placeholder-zinc-500 focus:border-zinc-400' 
                      : 'bg-zinc-50 border-zinc-300 text-zinc-900 placeholder-zinc-400 focus:border-zinc-500'"
                  />
                  <button 
                    @click="mirrorSubfolder = 'Indunilperamuna-Macos'; saveSubfolderSetting()"
                    title="Reset to default subfolder"
                    class="p-1 rounded-md border transition-colors shrink-0"
                    :class="isDarkMode ? 'bg-zinc-800 hover:bg-zinc-700 text-zinc-400 border-white/10' : 'bg-zinc-100 hover:bg-zinc-200 text-zinc-600 border-zinc-200'"
                  >
                    <RotateCcw class="w-3.5 h-3.5" />
                  </button>
                </div>
              </div>

              <!-- Live Full Destination Path Preview -->
              <div 
                class="p-2 rounded-md font-mono text-[10px] flex items-center justify-between border transition-colors"
                :class="isDarkMode ? 'bg-zinc-950/80 border-white/[0.06]' : 'bg-zinc-100/70 border-zinc-200'"
              >
                <div class="truncate max-w-[240px]">
                  <div class="text-[8px] uppercase tracking-wider font-sans font-bold opacity-60">Resolved Destination Path</div>
                  <div class="truncate font-semibold mt-0.5" :class="isDarkMode ? 'text-zinc-200' : 'text-zinc-800'">
                    {{ targetVolumePath }}/{{ mirrorSubfolder }}/
                  </div>
                </div>
                <button 
                  @click="openInFinder(`${targetVolumePath}/${mirrorSubfolder}`)" 
                  title="Reveal destination folder in Finder"
                  class="p-1 rounded hover:opacity-80 transition-opacity shrink-0"
                >
                  <ExternalLink class="w-3.5 h-3.5 opacity-60" />
                </button>
              </div>
            </div>

            <!-- Appearance / Theme Selector -->
            <div 
              class="border rounded-lg p-2.5 space-y-2 transition-colors"
              :class="isDarkMode ? 'bg-zinc-900/40 border-white/[0.06]' : 'bg-white border-zinc-200/80 shadow-xs'"
            >
              <div class="text-xs font-bold" :class="isDarkMode ? 'text-white' : 'text-zinc-900'">Appearance Theme</div>
              <div 
                class="grid grid-cols-3 gap-1 p-0.5 rounded-lg border transition-colors"
                :class="isDarkMode ? 'bg-zinc-950/60 border-white/[0.06]' : 'bg-zinc-200/60 border-zinc-200'"
              >
                <button 
                  @click="setTheme('system')" 
                  :class="themeChoice === 'system' 
                    ? (isDarkMode ? 'bg-zinc-800 text-white font-bold shadow-xs' : 'bg-white text-zinc-900 font-bold shadow-xs') 
                    : (isDarkMode ? 'text-zinc-400 hover:text-white' : 'text-zinc-600 hover:text-zinc-900')"
                  class="py-1 rounded text-[11px] flex items-center justify-center gap-1 transition-all"
                >
                  <Laptop class="w-3 h-3" />
                  <span>System</span>
                </button>
                <button 
                  @click="setTheme('light')" 
                  :class="themeChoice === 'light' 
                    ? (isDarkMode ? 'bg-zinc-800 text-white font-bold shadow-xs' : 'bg-white text-zinc-900 font-bold shadow-xs') 
                    : (isDarkMode ? 'text-zinc-400 hover:text-white' : 'text-zinc-600 hover:text-zinc-900')"
                  class="py-1 rounded text-[11px] flex items-center justify-center gap-1 transition-all"
                >
                  <Sun class="w-3 h-3 text-amber-500" />
                  <span>Light</span>
                </button>
                <button 
                  @click="setTheme('dark')" 
                  :class="themeChoice === 'dark' 
                    ? (isDarkMode ? 'bg-zinc-800 text-white font-bold shadow-xs' : 'bg-white text-zinc-900 font-bold shadow-xs') 
                    : (isDarkMode ? 'text-zinc-400 hover:text-white' : 'text-zinc-600 hover:text-zinc-900')"
                  class="py-1 rounded text-[11px] flex items-center justify-center gap-1 transition-all"
                >
                  <Moon class="w-3 h-3 text-blue-400" />
                  <span>Dark</span>
                </button>
              </div>
            </div>

            <!-- Permissions Shortcut: Only shown if action is required -->
            <div 
              v-if="!permissions.all_granted"
              class="border rounded-lg p-2.5 space-y-1.5 transition-colors"
              :class="isDarkMode ? 'bg-zinc-900/40 border-amber-500/30' : 'bg-amber-50/50 border-amber-300 shadow-xs'"
            >
              <div class="flex items-center justify-between">
                <span class="text-xs font-bold" :class="isDarkMode ? 'text-white' : 'text-zinc-900'">macOS Security & Permissions</span>
                <span 
                  class="text-[9px] font-bold font-mono px-1.5 py-0.5 rounded-full border"
                  :class="isDarkMode ? 'bg-amber-500/10 text-amber-300 border-amber-500/20' : 'bg-amber-100 text-amber-800 border-amber-200'"
                >
                  Action Required
                </span>
              </div>
              <button 
                @click="activeView = 'permissions'"
                class="w-full py-1 text-[11px] rounded font-semibold border flex items-center justify-center gap-1.5 transition-colors"
                :class="isDarkMode 
                  ? 'bg-zinc-800 hover:bg-zinc-700 text-zinc-200 border-white/10' 
                  : 'bg-zinc-100 hover:bg-zinc-200 text-zinc-800 border-zinc-200'"
              >
                <ShieldAlert class="w-3.5 h-3.5 text-amber-500" />
                <span>Grant Full Disk Access</span>
              </button>
            </div>

          </div>

        </div>

        <!-- Popover Footer Status Bar -->
        <div 
          class="px-3 py-1.5 border-t flex items-center justify-between text-[10px] shrink-0 font-mono transition-colors"
          :class="isDarkMode ? 'border-white/[0.06] bg-zinc-950/60 text-zinc-400' : 'border-zinc-200/80 bg-zinc-50 text-zinc-500'"
        >
          <button 
            @click="activeView = 'settings'" 
            class="truncate max-w-[220px] hover:underline text-left cursor-pointer"
            :title="`Subfolder: ${mirrorSubfolder} (Click to configure)`"
          >
            Subfolder: <span :class="isDarkMode ? 'text-zinc-200' : 'text-zinc-800'">{{ mirrorSubfolder }}</span>
          </button>
          <button 
            @click="openInFinder(`${targetVolumePath}/${mirrorSubfolder}`)"
            class="hover:underline flex items-center gap-1 shrink-0 font-medium"
            :class="isDarkMode ? 'text-zinc-300 hover:text-white' : 'text-zinc-700 hover:text-zinc-900'"
          >
            <span>Open Disc</span>
            <ExternalLink class="w-2.5 h-2.5" />
          </button>
        </div>
      </template>

    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import { 
  HardDrive, 
  Laptop, 
  FolderDown, 
  FolderUp, 
  Folder, 
  RefreshCw, 
  Zap, 
  Check, 
  ShieldCheck, 
  ShieldAlert,
  ArrowRightLeft, 
  ExternalLink, 
  Undo2, 
  FolderSearch,
  ChevronDown,
  Power,
  Settings,
  Sun,
  Moon,
  RotateCcw
} from 'lucide-vue-next';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

// View State: 'offload' | 'active' | 'settings' | 'permissions'
const activeView = ref('offload');
const isRefreshing = ref(false);
const isDriveConnected = ref(true);

// Theme State ('system' | 'light' | 'dark')
const themeChoice = ref(localStorage.getItem('offload_theme') || 'system');
const systemPrefersDark = ref(window.matchMedia('(prefers-color-scheme: dark)').matches);

// Listen to system theme changes
window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
  systemPrefersDark.value = e.matches;
  applyTheme();
});

const isDarkMode = computed(() => {
  if (themeChoice.value === 'dark') return true;
  if (themeChoice.value === 'light') return false;
  return systemPrefersDark.value;
});

function setTheme(theme) {
  themeChoice.value = theme;
  localStorage.setItem('offload_theme', theme);
  applyTheme();
}

function cycleTheme() {
  if (themeChoice.value === 'system') setTheme('light');
  else if (themeChoice.value === 'light') setTheme('dark');
  else setTheme('system');
}

function applyTheme() {
  if (isDarkMode.value) {
    document.documentElement.classList.add('dark');
  } else {
    document.documentElement.classList.remove('dark');
  }
}

// Permissions state
const permissions = ref({
  full_disk_access: false,
  external_drive_access: false,
  all_granted: false
});
const isCheckingPermissions = ref(false);

// Storage stats (Accurate /System/Volumes/Data)
const diskStats = ref({
  internal: { total: '460Gi', used: '397Gi', free: '34Gi', percent: 93 },
  external: { total: '578Gi', used: '188Gi', free: '390Gi', percent: 33 }
});

const internalUsedPercent = computed(() => diskStats.value.internal.percent);
const externalUsedPercent = computed(() => diskStats.value.external.percent);

// Target volume selection & mirror subfolder
const targetVolumePath = ref(localStorage.getItem('offload_target_volume') || '/Volumes/DataDisc2');
const mirrorSubfolder = ref(localStorage.getItem('offload_mirror_subfolder') || 'Indunilperamuna-Macos');

const availableVolumes = ref([
  { name: 'DataDisc2', path: '/Volumes/DataDisc2', free: '390Gi', total: '578Gi' },
  { name: 'DataDisc', path: '/Volumes/DataDisc', free: '2.3Gi', total: '353Gi' }
]);

const currentVolumeName = computed(() => {
  const match = availableVolumes.value.find(v => v.path === targetVolumePath.value);
  return match ? match.name : targetVolumePath.value.split('/').pop() || 'External Disc';
});

function saveSubfolderSetting() {
  if (!mirrorSubfolder.value.trim()) {
    mirrorSubfolder.value = 'Indunilperamuna-Macos';
  }
  localStorage.setItem('offload_mirror_subfolder', mirrorSubfolder.value.trim());
}

// Selected folder state
const selectedFolder = ref('');
const folderName = computed(() => selectedFolder.value.split('/').pop() || '');
const folderSize = ref('~');
const isDragging = ref(false);
const detectedSubitems = ref([]);
const selectedExceptions = ref([]);
const customExceptionInput = ref('');

// Processing state
const isProcessing = ref(false);
const processTitle = ref('Transferring files...');
const processStatusMessage = ref('Starting rsync engine...');
const processPercentage = ref(0);

// Active Symlinks
const activeSymlinks = ref([
  { name: 'spos', path: '/Users/indunilperamuna/Herd/spos', destination: '/Volumes/DataDisc2/Indunilperamuna-Macos/Herd/spos', size: '13 GB' },
  { name: 'Try Omarchy', path: '/Users/indunilperamuna/Library/Application Support/Try Omarchy', destination: '/Volumes/DataDisc2/Indunilperamuna-Macos/ApplicationSupport/Try Omarchy', size: '30 GB' },
  { name: 'Android SDK', path: '/Users/indunilperamuna/Library/Android/sdk', destination: '/Volumes/DataDisc2/Indunilperamuna-Macos/Android/sdk', size: '14 GB' },
  { name: 'Android AVD', path: '/Users/indunilperamuna/.android/avd', destination: '/Volumes/DataDisc2/Indunilperamuna-Macos/Android/avd', size: '5.9 GB' },
  { name: 'MobileSync (iOS)', path: '/Users/indunilperamuna/Library/Application Support/MobileSync/Backup', destination: '/Volumes/DataDisc2/Indunilperamuna-Macos/MobileSync/Backup', size: '11 GB' },
  { name: 'Ollama Models', path: '/Users/indunilperamuna/.ollama/models', destination: '/Volumes/DataDisc2/Indunilperamuna-Macos/ollama/models', size: '5.4 GB' },
  { name: 'OrbStack (Docker)', path: '/Users/indunilperamuna/Library/Group Containers/HUAQ24HBR6.dev.orbstack/data', destination: '/Volumes/DataDisc2/Indunilperamuna-Macos/OrbStackData/data', size: '13 GB' }
]);

// Check Permissions
async function checkSystemPermissions() {
  isCheckingPermissions.value = true;
  try {
    const status = await invoke('check_permissions', { targetMount: targetVolumePath.value });
    if (status) {
      permissions.value = status;
      if (status.all_granted && activeView.value === 'permissions') {
        activeView.value = 'offload';
      }
    }
  } catch (err) {
    console.warn('Permission check error:', err);
  } finally {
    isCheckingPermissions.value = false;
  }
}

async function verifyPermissionsLive() {
  await checkSystemPermissions();
}

async function openSettings(pane) {
  try {
    await invoke('open_privacy_settings', { pane });
  } catch (err) {
    console.error('Failed to open settings:', err);
  }
}

function dismissPermissionsScreen() {
  localStorage.setItem('offload_permissions_dismissed', 'true');
  activeView.value = 'offload';
}

async function refreshDiskStats() {
  isRefreshing.value = true;
  try {
    const vols = await invoke('list_mounted_volumes');
    if (vols && vols.length) {
      availableVolumes.value = vols;
      if (!vols.some(v => v.path === targetVolumePath.value)) {
        targetVolumePath.value = vols[0].path;
      }
    }

    const stats = await invoke('get_disk_stats', { targetMount: targetVolumePath.value });
    if (stats) {
      diskStats.value = stats;
      isDriveConnected.value = stats.external_connected ?? true;
    }
  } catch (err) {
    console.warn('Stats fetch warning:', err);
  } finally {
    setTimeout(() => { isRefreshing.value = false; }, 350);
  }
}

function onVolumeChange() {
  localStorage.setItem('offload_target_volume', targetVolumePath.value);
  refreshDiskStats();
  checkSystemPermissions();
}

async function browseCustomVolume() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Destination Drive or Folder'
    });
    if (selected) {
      targetVolumePath.value = selected;
      localStorage.setItem('offload_target_volume', selected);
      const volName = selected.split('/').pop() || 'Custom Disk';
      if (!availableVolumes.value.some(v => v.path === selected)) {
        availableVolumes.value.push({ name: volName, path: selected, free: '~', total: '~' });
      }
      refreshDiskStats();
    }
  } catch (err) {
    console.error(err);
  }
}

async function selectFolderDialog() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Choose Folder to Offload to External Drive'
    });
    if (selected) {
      handleFolderSelection(selected);
    }
  } catch (err) {
    console.error('File dialog error:', err);
  }
}

function handleFolderDrop(e) {
  isDragging.value = false;
  const files = e.dataTransfer.files;
  if (files && files.length > 0) {
    handleFolderSelection(files[0].path);
  }
}

async function handleFolderSelection(path) {
  selectedFolder.value = path;
  folderSize.value = 'Calculating...';
  detectedSubitems.value = [];
  selectedExceptions.value = [];

  try {
    const res = await invoke('scan_folder', { path });
    if (res) {
      folderSize.value = res.size;
      detectedSubitems.value = res.items || [];
      // Full folder offload by default (exceptions are purely optional manual selections)
      selectedExceptions.value = [];
    }
  } catch (err) {
    folderSize.value = 'Unknown';
    console.error('Scan error:', err);
  }
}

function toggleException(name) {
  if (selectedExceptions.value.includes(name)) {
    selectedExceptions.value = selectedExceptions.value.filter(item => item !== name);
  } else {
    selectedExceptions.value.push(name);
  }
}

function addCustomException() {
  const trimmed = customExceptionInput.value.trim();
  if (trimmed && !selectedExceptions.value.includes(trimmed)) {
    selectedExceptions.value.push(trimmed);
    if (!detectedSubitems.value.some(i => i.name === trimmed)) {
      detectedSubitems.value.push({ name: trimmed, size: '~', is_recommended: true });
    }
    customExceptionInput.value = '';
  }
}

function resetSelectedFolder() {
  selectedFolder.value = '';
  detectedSubitems.value = [];
  selectedExceptions.value = [];
}

async function startOffloadAction() {
  if (!selectedFolder.value) return;

  const destBase = `${targetVolumePath.value}/${mirrorSubfolder.value}`;

  isProcessing.value = true;
  processTitle.value = `Offloading ${folderName.value}...`;
  processStatusMessage.value = `Transferring to ${currentVolumeName.value}/${mirrorSubfolder.value}...`;
  processPercentage.value = 25;

  try {
    const exceptStr = selectedExceptions.value.join(',');
    await invoke('offload_folder', { 
      path: selectedFolder.value, 
      exceptList: exceptStr,
      destBase: destBase
    });
    
    processPercentage.value = 100;
    processStatusMessage.value = 'Symlink established!';
    setTimeout(() => {
      isProcessing.value = false;
      refreshDiskStats();
      scanActiveSymlinks();
      activeView.value = 'active';
      resetSelectedFolder();
    }, 1200);
  } catch (err) {
    processStatusMessage.value = `Error: ${err}`;
    setTimeout(() => { isProcessing.value = false; }, 3000);
  }
}

async function reverseItemAction(path) {
  if (!confirm(`Bring ${path} back from ${currentVolumeName.value} to your internal SSD?`)) return;

  try {
    await invoke('reverse_folder', { path });
    await refreshDiskStats();
    await scanActiveSymlinks();
  } catch (err) {
    alert(`Failed to reverse: ${err}`);
  }
}

async function openInFinder(path) {
  try {
    await invoke('open_in_finder', { path });
  } catch (err) {
    console.error(err);
  }
}

async function scanActiveSymlinks() {
  try {
    const list = await invoke('list_offloaded_items', { targetMount: targetVolumePath.value });
    if (list && list.length) {
      activeSymlinks.value = list;
    }
  } catch (err) {
    console.warn(err);
  }
}

async function quitApplication() {
  try {
    await invoke('quit_app');
  } catch {
    window.close();
  }
}

onMounted(async () => {
  applyTheme();
  await checkSystemPermissions();
  if (!permissions.value.all_granted) {
    activeView.value = 'permissions';
  } else {
    activeView.value = 'offload';
  }
  refreshDiskStats();
  scanActiveSymlinks();
});
</script>
