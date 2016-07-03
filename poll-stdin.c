#include <unistd.h>
#include <stdio.h>
#include <sys/event.h>
#include <mach/mach_time.h>
#include <termios.h>

struct keyboard_config {
  int event_queue;
  struct kevent event_descriptor;
  mach_timebase_info_data_t time_conversion_info;
  struct termios original_term_config;
  struct termios current_term_config;
};

struct poll_result {
  uint32_t key;
  uint32_t elapsed_ns;
};

int init_keyboard(struct keyboard_config * kbconfig) {
  kbconfig->event_queue = kqueue();
  if(kbconfig->event_queue == -1) {
    return -1;
  }
  //set fields in kevent struct
  kbconfig->event_descriptor.ident  = STDIN_FILENO;
  kbconfig->event_descriptor.filter = EVFILT_READ;
  kbconfig->event_descriptor.flags  = EV_ADD;
  kbconfig->event_descriptor.fflags = 0;
  kbconfig->event_descriptor.data   = 0;
  kbconfig->event_descriptor.udata  = NULL;
  //and pass changes to kernel
  kevent(kbconfig->event_queue, &kbconfig->event_descriptor, 1, NULL, 0, NULL);
  mach_timebase_info(&kbconfig->time_conversion_info);

  tcgetattr(STDIN_FILENO, &kbconfig->original_term_config);
  kbconfig->current_term_config = kbconfig->original_term_config;
  cfmakeraw(&kbconfig->current_term_config);
  tcsetattr(STDIN_FILENO, 0, &kbconfig->current_term_config);

  return 0;
}

int deinit_keyboard(struct keyboard_config * kbconfig) {
  close(kbconfig->event_queue);

  tcsetattr(STDIN_FILENO, 0, &kbconfig->original_term_config);

  return 0;
}

uint64_t mach_absolute_time_diff_to_ns(const mach_timebase_info_data_t * mt_info, uint64_t start_time_mtu, uint64_t end_time_mtu) {
  uint64_t diff_mtu = end_time_mtu - start_time_mtu;
  return (uint64_t)((double)diff_mtu * (double)mt_info->numer / (double)mt_info->denom);
}

struct poll_result poll_keyboard(struct keyboard_config * kbconfig, uint32_t timeout_ns) {
  struct timespec timeout;
  timeout.tv_sec = 0;
  timeout.tv_nsec = timeout_ns;

  uint64_t start_time_mtu = mach_absolute_time();

  int change_count = kevent(kbconfig->event_queue,
    NULL, 0,
    &kbconfig->event_descriptor, 1,
    &timeout);

  uint64_t end_time_mtu = mach_absolute_time();
  uint32_t elapsed_ns = (uint32_t)mach_absolute_time_diff_to_ns(
    &kbconfig->time_conversion_info,
    start_time_mtu, end_time_mtu);

  struct poll_result result = {0, elapsed_ns};
  
  if(change_count != 0) {
    read(STDIN_FILENO, &result.key, sizeof(result.key));
  }

  return result;
}

int main() {
  struct keyboard_config kbconfig;
  init_keyboard(&kbconfig);
  int should_exit = 0;
  //clear screen
  printf("\033[2J");
  //hide cursor
  printf("\033[?25l");
  fflush(stdout);
  const uint32_t interval_ns = 500000000;

  uint32_t counter = 0;
  
  while(!should_exit) {
    ++counter;
    uint32_t remaining_ns = interval_ns;
    while(remaining_ns > 0) {
      struct poll_result result = poll_keyboard(&kbconfig, remaining_ns);
      printf("\033[0;0Hit:%5d ", counter);
      if(result.key != 0) {
        printf("\033[0;10Hlast key: %#010x", result.key);
      }
      printf("\033[0;30H elapsed_ns: %10d ns\n", result.elapsed_ns);

      if(result.elapsed_ns > remaining_ns) {
        remaining_ns = 0;
      }
      else {
        remaining_ns -= result.elapsed_ns;
      }

      if(result.key == 27) {
        should_exit = 1;
      }

      fflush(stdout);

    }

  }


  deinit_keyboard(&kbconfig);

  printf("\nexiting ...\n");
  fflush(stdout);
}
