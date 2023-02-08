// Generated by gir (https://github.com/gtk-rs/gir @ 425f84d5af7f)
// from 
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 4eaad6a722bf)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) ADW_ANIMATION_FINISHED);
    PRINT_CONSTANT((gint) ADW_ANIMATION_IDLE);
    PRINT_CONSTANT((gint) ADW_ANIMATION_PAUSED);
    PRINT_CONSTANT((gint) ADW_ANIMATION_PLAYING);
    PRINT_CONSTANT((gint) ADW_CENTERING_POLICY_LOOSE);
    PRINT_CONSTANT((gint) ADW_CENTERING_POLICY_STRICT);
    PRINT_CONSTANT((gint) ADW_COLOR_SCHEME_DEFAULT);
    PRINT_CONSTANT((gint) ADW_COLOR_SCHEME_FORCE_DARK);
    PRINT_CONSTANT((gint) ADW_COLOR_SCHEME_FORCE_LIGHT);
    PRINT_CONSTANT((gint) ADW_COLOR_SCHEME_PREFER_DARK);
    PRINT_CONSTANT((gint) ADW_COLOR_SCHEME_PREFER_LIGHT);
    PRINT_CONSTANT(ADW_DURATION_INFINITE);
    PRINT_CONSTANT((gint) ADW_EASE_IN_BACK);
    PRINT_CONSTANT((gint) ADW_EASE_IN_BOUNCE);
    PRINT_CONSTANT((gint) ADW_EASE_IN_CIRC);
    PRINT_CONSTANT((gint) ADW_EASE_IN_CUBIC);
    PRINT_CONSTANT((gint) ADW_EASE_IN_ELASTIC);
    PRINT_CONSTANT((gint) ADW_EASE_IN_EXPO);
    PRINT_CONSTANT((gint) ADW_EASE_IN_OUT_BACK);
    PRINT_CONSTANT((gint) ADW_EASE_IN_OUT_BOUNCE);
    PRINT_CONSTANT((gint) ADW_EASE_IN_OUT_CIRC);
    PRINT_CONSTANT((gint) ADW_EASE_IN_OUT_CUBIC);
    PRINT_CONSTANT((gint) ADW_EASE_IN_OUT_ELASTIC);
    PRINT_CONSTANT((gint) ADW_EASE_IN_OUT_EXPO);
    PRINT_CONSTANT((gint) ADW_EASE_IN_OUT_QUAD);
    PRINT_CONSTANT((gint) ADW_EASE_IN_OUT_QUART);
    PRINT_CONSTANT((gint) ADW_EASE_IN_OUT_QUINT);
    PRINT_CONSTANT((gint) ADW_EASE_IN_OUT_SINE);
    PRINT_CONSTANT((gint) ADW_EASE_IN_QUAD);
    PRINT_CONSTANT((gint) ADW_EASE_IN_QUART);
    PRINT_CONSTANT((gint) ADW_EASE_IN_QUINT);
    PRINT_CONSTANT((gint) ADW_EASE_IN_SINE);
    PRINT_CONSTANT((gint) ADW_EASE_OUT_BACK);
    PRINT_CONSTANT((gint) ADW_EASE_OUT_BOUNCE);
    PRINT_CONSTANT((gint) ADW_EASE_OUT_CIRC);
    PRINT_CONSTANT((gint) ADW_EASE_OUT_CUBIC);
    PRINT_CONSTANT((gint) ADW_EASE_OUT_ELASTIC);
    PRINT_CONSTANT((gint) ADW_EASE_OUT_EXPO);
    PRINT_CONSTANT((gint) ADW_EASE_OUT_QUAD);
    PRINT_CONSTANT((gint) ADW_EASE_OUT_QUART);
    PRINT_CONSTANT((gint) ADW_EASE_OUT_QUINT);
    PRINT_CONSTANT((gint) ADW_EASE_OUT_SINE);
    PRINT_CONSTANT((gint) ADW_FLAP_FOLD_POLICY_ALWAYS);
    PRINT_CONSTANT((gint) ADW_FLAP_FOLD_POLICY_AUTO);
    PRINT_CONSTANT((gint) ADW_FLAP_FOLD_POLICY_NEVER);
    PRINT_CONSTANT((gint) ADW_FLAP_TRANSITION_TYPE_OVER);
    PRINT_CONSTANT((gint) ADW_FLAP_TRANSITION_TYPE_SLIDE);
    PRINT_CONSTANT((gint) ADW_FLAP_TRANSITION_TYPE_UNDER);
    PRINT_CONSTANT((gint) ADW_FOLD_THRESHOLD_POLICY_MINIMUM);
    PRINT_CONSTANT((gint) ADW_FOLD_THRESHOLD_POLICY_NATURAL);
    PRINT_CONSTANT((gint) ADW_LEAFLET_TRANSITION_TYPE_OVER);
    PRINT_CONSTANT((gint) ADW_LEAFLET_TRANSITION_TYPE_SLIDE);
    PRINT_CONSTANT((gint) ADW_LEAFLET_TRANSITION_TYPE_UNDER);
    PRINT_CONSTANT((gint) ADW_LINEAR);
    PRINT_CONSTANT((gint) ADW_NAVIGATION_DIRECTION_BACK);
    PRINT_CONSTANT((gint) ADW_NAVIGATION_DIRECTION_FORWARD);
    PRINT_CONSTANT((gint) ADW_RESPONSE_DEFAULT);
    PRINT_CONSTANT((gint) ADW_RESPONSE_DESTRUCTIVE);
    PRINT_CONSTANT((gint) ADW_RESPONSE_SUGGESTED);
    PRINT_CONSTANT((gint) ADW_SQUEEZER_TRANSITION_TYPE_CROSSFADE);
    PRINT_CONSTANT((gint) ADW_SQUEEZER_TRANSITION_TYPE_NONE);
    PRINT_CONSTANT((guint) ADW_TAB_VIEW_SHORTCUT_ALL_SHORTCUTS);
    PRINT_CONSTANT((guint) ADW_TAB_VIEW_SHORTCUT_ALT_DIGITS);
    PRINT_CONSTANT((guint) ADW_TAB_VIEW_SHORTCUT_ALT_ZERO);
    PRINT_CONSTANT((guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_END);
    PRINT_CONSTANT((guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_HOME);
    PRINT_CONSTANT((guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_PAGE_DOWN);
    PRINT_CONSTANT((guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_PAGE_UP);
    PRINT_CONSTANT((guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_END);
    PRINT_CONSTANT((guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_HOME);
    PRINT_CONSTANT((guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_PAGE_DOWN);
    PRINT_CONSTANT((guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_PAGE_UP);
    PRINT_CONSTANT((guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_SHIFT_TAB);
    PRINT_CONSTANT((guint) ADW_TAB_VIEW_SHORTCUT_CONTROL_TAB);
    PRINT_CONSTANT((guint) ADW_TAB_VIEW_SHORTCUT_NONE);
    PRINT_CONSTANT((gint) ADW_TOAST_PRIORITY_HIGH);
    PRINT_CONSTANT((gint) ADW_TOAST_PRIORITY_NORMAL);
    PRINT_CONSTANT((gint) ADW_VIEW_SWITCHER_POLICY_NARROW);
    PRINT_CONSTANT((gint) ADW_VIEW_SWITCHER_POLICY_WIDE);
    return 0;
}
