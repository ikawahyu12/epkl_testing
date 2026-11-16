import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import com.kms.katalon.core.llm.keyword.LlmKeywords as LLM
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import org.openqa.selenium.Keys as Keys
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser(null)

WebUI.navigateToUrl('http://localhost/e-pkl-baru/admin-web/public/?page=data-siswa')

WebUI.setText(findTestObject('Data_siswa_CRUD/Page_Login Admin  E-PKL/input_Masukkan username atau email'), 'admin')

WebUI.setEncryptedText(findTestObject('Data_siswa_CRUD/Page_Login Admin  E-PKL/input_Masukkan password'), '8SQVv/p9jVQa8N8i3eDN3paNTjSkKFPY')

WebUI.click(findTestObject('Data_siswa_CRUD/Page_Login Admin  E-PKL/button_Masuk'))

WebUI.click(findTestObject('Data_siswa_CRUD/Page_Dashboard Monitoring PKL  E-PKL/a_Data Siswa'))

WebUI.click(findTestObject('Data_siswa_CRUD/Page_Data Siswa PKL  E-PKL/button_Hapus'))

WebUI.click(findTestObject('Data_siswa_CRUD/Page_Data Siswa PKL  E-PKL/button_Hapus_1'))

