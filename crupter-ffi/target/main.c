#include "crupter.h"
#include <stdio.h>
#include <unistd.h>

int main() {
    clean_hash_files();
    add_hash_file("/Users/mig/Downloads/CODE_icseseipfinal.zip");
    add_hash_file("/Users/mig/Downloads/iMazing2forMac.dmg");
    run_hash_session();
    while (1) {
        if (hash_running_count() >= 1) {
            StatusInfo info = get_hash_progress(0);
            print_info(info);
            free_status_info(info);
            if (info.status == 1) {
                break;
            }
            sleep(1);
        }
    }
}