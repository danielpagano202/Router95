#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <windows.h>

#define MAX_LINE_LENGTH 4096

void get_script_dir(char* buffer, size_t buffer_size) {
    GetModuleFileNameA(NULL, buffer, buffer_size);
    char* last_slash = strrchr(buffer, '\\');
    if (last_slash != NULL) {
        *last_slash = '\0';
    }
}

void escape_json(const char* input, char* output, size_t output_size) {
    char* dst = output;
    char* dst_end = output + output_size - 1;

    for (const char* src = input; *src != '\0' && dst < dst_end; src++) {
        if (*src == '\\' || *src == '"') {
            if (dst + 2 > dst_end) {
                break;
            }
            *dst++ = '\\';
            *dst++ = *src;
        } else if (*src == '\n' || *src == '\r' || *src == '\t') {
            if (dst + 2 > dst_end) {
                break;
            }
            *dst++ = '\\';
            *dst++ = (*src == '\n') ? 'n' : (*src == '\r') ? 'r' : 't';
        } else {
            *dst++ = *src;
        }
    }

    *dst = '\0';
}

int main() {
    char script_dir[MAX_PATH];
    char html_path[MAX_PATH];
    char escaped_html_path[MAX_PATH * 2];
    char template_data[MAX_LINE_LENGTH];
    char escaped_template_data[MAX_LINE_LENGTH * 2];
    char json_response[MAX_LINE_LENGTH * 4];
    char line[MAX_LINE_LENGTH];

    while (fgets(line, sizeof(line), stdin) != NULL) {
        get_script_dir(script_dir, sizeof(script_dir));
        snprintf(html_path, sizeof(html_path), "%s\\index.html", script_dir);
        escape_json(html_path, escaped_html_path, sizeof(escaped_html_path));

        snprintf(
            template_data,
            sizeof(template_data),
            "{\"templatePath\":\"%s\",\"arguments\":{\"language\":\"C\"}}",
            escaped_html_path
        );
        escape_json(template_data, escaped_template_data, sizeof(escaped_template_data));

        snprintf(
            json_response,
            sizeof(json_response),
            "{\"responseType\":\"template\",\"responseCode\":200,\"data\":\"%s\"}",
            escaped_template_data
        );

        printf("%s\n", json_response);
        fflush(stdout);
    }

    return 0;
}
