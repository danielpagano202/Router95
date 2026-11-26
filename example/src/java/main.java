import java.io.BufferedReader;
import java.io.File;
import java.io.InputStreamReader;

public class main {
    public static void main(String[] args) {
        try (BufferedReader reader = new BufferedReader(new InputStreamReader(System.in))) {
            String line;

            while ((line = reader.readLine()) != null) {
                try {
                    String scriptDir = new File(main.class.getProtectionDomain()
                            .getCodeSource()
                            .getLocation()
                            .toURI())
                            .toString();

                    File htmlFile = new File(scriptDir, "index.html");
                    String htmlPath = htmlFile.getAbsolutePath();

                    // Escape backslashes twice (4 backslashes) so they survive both JSON parses
                    String escapedPath = htmlPath.replace("\\", "\\\\\\\\");

                    // Escape inner quotes so innerData is a valid JSON string inside "data"
                    String innerData = """
                            {\\"templatePath\\":\\"%s\\",\\"arguments\\":{\\"language\\":\\"Java\\"}}
                            """.strip().formatted(escapedPath);

                    String jsonResponse = """
                            {"responseType":"%s","responseCode":%d,"data":"%s"}
                            """.strip().formatted("template", 200, innerData);

                    System.out.println(jsonResponse);
                    System.out.flush();

                } catch (Exception e) {
                    System.out.println("ERROR: " + e.getMessage());
                    System.out.flush();
                }
            }
        } catch (Exception e) {
            System.out.println("ERROR: Failed to initialize stdin reader: " + e.getMessage());
            System.out.flush();
        }
    }
}
