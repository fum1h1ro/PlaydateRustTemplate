#include <pd_api.h>

extern int rust_event_handler(PlaydateAPI *pd, PDSystemEvent event, uint32_t arg);
extern int rust_update(PlaydateAPI *pd);

static PlaydateAPI *pd_api;
static int update(void *arg)
{
    return rust_update(pd_api);
}

int eventHandler(PlaydateAPI *pd, PDSystemEvent event, uint32_t arg)
{
    pd->system->logToConsole("API: %08x, Event: %d, Arg: %d", pd, event, arg);
    if (event == kEventInit) {
        pd_api = pd;
        pd->system->setUpdateCallback(update, NULL);
    }
    return rust_event_handler(pd, event, arg);
}



