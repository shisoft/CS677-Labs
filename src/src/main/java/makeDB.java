
import java.io.FileWriter;
import java.io.IOException;
import java.util.Arrays;
import java.util.List;

public class makeDB {
        public static void main(String[] args) throws IOException {

            List<List<String>> rows = Arrays.asList(
                    Arrays.asList("How to get a good grade in 677 in 20 minutes a day", "1", "10", "19.9","distributed systems"),
                    Arrays.asList("RPCs for Dummies", "2", "10", "14.9","distributed systems"),
                    Arrays.asList("Xen and the Art of Surviving Graduate School", "3", "10", "29.9","graduate school"),
                    Arrays.asList("Cooking for the Impatient Graduate Student", "4", "10", "39.9","graduate school")
                    );

            FileWriter csvWriter = new FileWriter("inventory.csv");
            csvWriter.append("name");
            csvWriter.append(",");
            csvWriter.append("id");
            csvWriter.append(",");
            csvWriter.append("stock");
            csvWriter.append(",");
            csvWriter.append("price");
            csvWriter.append(",");
            csvWriter.append("topic");
            csvWriter.append("\n");

            for (List<String> rowData : rows) {
                csvWriter.append(String.join(",", rowData));
                csvWriter.append("\n");
            }

            csvWriter.flush();
            csvWriter.close();
        }
}
